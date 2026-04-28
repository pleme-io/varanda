// Yew view components for the three saguão view modes.

use crate::hostname::ViewMode;
use crate::model::AccessibleService;
use std::collections::BTreeMap;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PortalProps {
    pub mode: ViewMode,
    pub user_display: String,
    pub services: Vec<AccessibleService>,
}

#[function_component(Portal)]
pub fn portal(props: &PortalProps) -> Html {
    let filtered = filter_for_mode(&props.services, &props.mode);
    let title = title_for_mode(&props.mode);

    html! {
        <main class="varanda-root">
            <header class="varanda-header">
                <h1>{ title }</h1>
                <span class="varanda-user">{ &props.user_display }</span>
            </header>
            <Tiles services={filtered} mode={props.mode.clone()} />
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct TilesProps {
    services: Vec<AccessibleService>,
    mode: ViewMode,
}

#[function_component(Tiles)]
fn tiles(props: &TilesProps) -> Html {
    if props.services.is_empty() {
        return html! {
            <p class="varanda-empty">
                { "No services accessible to you in this view. " }
                { "If you think this is a mistake, ask the operator to update your access policy." }
            </p>
        };
    }

    // Group by (location, cluster) so the rendered grid is structured.
    let mut grouped: BTreeMap<(String, String), Vec<&AccessibleService>> = BTreeMap::new();
    for s in &props.services {
        grouped
            .entry((s.location.clone(), s.cluster.clone()))
            .or_default()
            .push(s);
    }

    html! {
        <div class="varanda-tiles">
            { for grouped.iter().map(|((location, cluster), svcs)| {
                let group_label = match props.mode {
                    ViewMode::Fleet => format!("{location} / {cluster}"),
                    ViewMode::Location { .. } => cluster.clone(),
                    ViewMode::Cluster { .. } => String::new(),
                };
                html! {
                    <section class="varanda-group">
                        if !group_label.is_empty() {
                            <h2>{ group_label }</h2>
                        }
                        <div class="varanda-tile-grid">
                            { for svcs.iter().map(|s| html! { <Tile service={(*s).clone()} /> }) }
                        </div>
                    </section>
                }
            }) }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TileProps {
    service: AccessibleService,
}

#[function_component(Tile)]
fn tile(props: &TileProps) -> Html {
    let href = format!("https://{}/", props.service.hostname);
    html! {
        <a class="varanda-tile" href={href}>
            <div class="varanda-tile-name">{ &props.service.display_name }</div>
            if let Some(desc) = &props.service.description {
                <div class="varanda-tile-desc">{ desc }</div>
            }
            <div class="varanda-tile-host">{ &props.service.hostname }</div>
        </a>
    }
}

fn filter_for_mode(svcs: &[AccessibleService], mode: &ViewMode) -> Vec<AccessibleService> {
    match mode {
        ViewMode::Fleet => svcs.to_vec(),
        ViewMode::Location { location } => svcs
            .iter()
            .filter(|s| &s.location == location)
            .cloned()
            .collect(),
        ViewMode::Cluster { cluster, location } => svcs
            .iter()
            .filter(|s| &s.cluster == cluster && &s.location == location)
            .cloned()
            .collect(),
    }
}

fn title_for_mode(mode: &ViewMode) -> String {
    match mode {
        ViewMode::Fleet => "quero.cloud — your homelab fleet".into(),
        ViewMode::Location { location } => format!("{location} — clusters and services"),
        ViewMode::Cluster {
            cluster, location, ..
        } => format!("{cluster}.{location} — services"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svc(slug: &str, cluster: &str, location: &str) -> AccessibleService {
        AccessibleService {
            slug: slug.into(),
            display_name: slug.into(),
            cluster: cluster.into(),
            location: location.into(),
            icon: None,
            description: None,
            hostname: format!("{slug}.{cluster}.{location}.quero.cloud"),
        }
    }

    #[test]
    fn fleet_filter_keeps_all() {
        let svcs = vec![
            svc("vault", "rio", "bristol"),
            svc("photos", "mar", "parnamirim"),
        ];
        assert_eq!(filter_for_mode(&svcs, &ViewMode::Fleet).len(), 2);
    }

    #[test]
    fn location_filter_narrows() {
        let svcs = vec![
            svc("vault", "rio", "bristol"),
            svc("photos", "mar", "parnamirim"),
        ];
        let f = filter_for_mode(
            &svcs,
            &ViewMode::Location {
                location: "bristol".into(),
            },
        );
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].slug, "vault");
    }

    #[test]
    fn cluster_filter_narrows_more() {
        let svcs = vec![
            svc("vault", "rio", "bristol"),
            svc("photos", "rio", "bristol"),
            svc("photos", "mar", "parnamirim"),
        ];
        let f = filter_for_mode(
            &svcs,
            &ViewMode::Cluster {
                cluster: "rio".into(),
                location: "bristol".into(),
            },
        );
        assert_eq!(f.len(), 2);
    }
}
