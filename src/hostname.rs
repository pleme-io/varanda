// Hostname → ViewMode router. Same parse logic as vigia, kept
// in-house to avoid pulling vigia (a server crate) into a WASM build.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewMode {
    /// quero.cloud — apex; renders all locations.
    Fleet,
    /// <location>.quero.cloud — renders all clusters at this location.
    Location { location: String },
    /// <cluster>.<location>.quero.cloud — renders all services in this cluster.
    Cluster {
        cluster: String,
        location: String,
    },
}

/// Reserved single-label prefixes that map to Fleet view rather than
/// being treated as a location name. `www` is the canonical web alias
/// for the apex; future entries (e.g., `app`, `home`) can be added
/// here without changing call sites.
const FLEET_ALIASES: &[&str] = &["www", "app", "home"];

#[must_use]
pub fn from_host(host: &str) -> ViewMode {
    let labels: Vec<&str> = host.split('.').collect();
    // Trailing two labels are the TLD (e.g., "quero.cloud").
    // Anything before that determines the view.
    if labels.len() <= 2 {
        return ViewMode::Fleet;
    }
    let prefix = &labels[..labels.len() - 2];
    match prefix.len() {
        0 => ViewMode::Fleet, // shouldn't happen given len > 2 check
        1 if FLEET_ALIASES.contains(&prefix[0]) => ViewMode::Fleet,
        1 => ViewMode::Location {
            location: prefix[0].into(),
        },
        2 => ViewMode::Cluster {
            cluster: prefix[0].into(),
            location: prefix[1].into(),
        },
        _ => {
            // 3+ prefix labels = a workload service hostname (e.g.,
            // vault.rio.bristol.quero.cloud) — we shouldn't be
            // running varanda there, but render fleet view as a fallback.
            ViewMode::Fleet
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apex_is_fleet() {
        assert_eq!(from_host("quero.cloud"), ViewMode::Fleet);
    }

    #[test]
    fn one_prefix_is_location() {
        assert_eq!(
            from_host("bristol.quero.cloud"),
            ViewMode::Location {
                location: "bristol".into()
            }
        );
    }

    #[test]
    fn two_prefix_is_cluster() {
        assert_eq!(
            from_host("rio.bristol.quero.cloud"),
            ViewMode::Cluster {
                cluster: "rio".into(),
                location: "bristol".into()
            }
        );
    }

    #[test]
    fn workload_hostname_falls_back_to_fleet() {
        assert_eq!(
            from_host("vault.rio.bristol.quero.cloud"),
            ViewMode::Fleet
        );
    }

    #[test]
    fn www_alias_resolves_to_fleet() {
        assert_eq!(from_host("www.quero.cloud"), ViewMode::Fleet);
    }

    #[test]
    fn other_fleet_aliases_resolve_to_fleet() {
        assert_eq!(from_host("app.quero.cloud"), ViewMode::Fleet);
        assert_eq!(from_host("home.quero.cloud"), ViewMode::Fleet);
    }

    #[test]
    fn non_alias_single_prefix_is_still_location() {
        assert_eq!(
            from_host("bristol.quero.cloud"),
            ViewMode::Location { location: "bristol".into() }
        );
    }
}
