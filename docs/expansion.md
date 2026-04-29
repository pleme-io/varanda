# varanda — expansion playbook

> **Premise:** every recurring shape in the saguão fleet should be
> one Lisp form (cracha) + one chart values change (lareira) +
> zero hand-edits. varanda's job is to render the resulting world
> as a portal. The five expansions below cover everything we
> currently anticipate.

## 1. Add a new family member

**No varanda change needed.** Once the operator (or the future admin
PWA) updates the relevant `(defcrachá :members [...])` form, varanda's
`accessible_services` query reflects the new member's view on next
sign-in. Their portal renders with the tiles their grants entitle them
to. **Zero front-end work per member.**

## 2. Add a new service on an existing cluster

**No varanda change needed.** The cracha-controller watches HelmReleases
labeled `app.kubernetes.io/part-of=saguao-service` and auto-derives
ServiceCatalog entries (see `theory/SAGUAO.md` §V.1.6). varanda's
`/accessible-services` query picks up the new service on next render.

The service's `display-name`, `icon`, `description` come from
`saguao.pleme.io/*` annotations on the HelmRelease. Tiles render
automatically with the right copy.

## 3. Add a new cluster

Three things change, all mechanical:

1. **`(defcluster …)`** Lisp form authored under `pleme-io/cracha/examples/<cluster>.lisp`.
   `cracha render cluster …` emits the 4 deployable artifacts
   (see `theory/SAGUAO.md` §V.1).
2. **varanda Pages custom-domains list extended** in
   `pangea-architectures/workspaces/cloudflare-pleme/domains/quero.cloud.yaml`
   under `pages_apps[varanda].custom_domains`. Add `<location>` and
   `<cluster>.<location>` entries. The pangea-operator on rio
   reconciles the new Cloudflare Pages domain bindings via the
   InfrastructureTemplate (no workstation `pangea apply`).
3. **varanda recompiles + redeploys** — the Yew bundle's
   hostname-router (`src/hostname.rs`) uses the runtime
   `window.location.host`, so no per-cluster code lands in the
   bundle. Same artifact serves every hostname.

After a hard refresh: the new `<cluster>.<location>.quero.cloud` URL
loads varanda; the cluster-view filter (`ViewMode::Cluster`) renders
just that cluster's services.

## 4. Add a new location

Same as adding a cluster — the location is implicit in the `(defcluster
:location …)` field. The `<location>.quero.cloud` Pages custom-domain
entry is what makes the location-view render directly.

A location with multiple clusters in the future requires no extra
varanda change; the `ViewMode::Location` filter automatically groups
multiple clusters under the location header.

## 5. Add a new view mode

When the saguão portal needs a view that isn't fleet/location/cluster:

1. **Extend `ViewMode`** in `src/hostname.rs` with the new variant.
   Add the hostname recognizer (with tests).
2. **Author the Yew component** under `src/view.rs` (or
   `src/view/<mode>.rs` if views are split). Re-use the existing
   `varanda-root` / `varanda-header` / `varanda-tile-grid`
   primitives.
3. **Update `Portal::title_for_mode` + `filter_for_mode`** in
   `src/view.rs`.
4. **(Optional)** if the new view needs a different tile shape,
   add the CSS class to `public/industrial.css` (consuming ishou
   tokens exclusively).

Anticipated future view modes:

- `ServiceDetail { app, cluster, location }` — landing on
  `vault.rio.bristol.quero.cloud/varanda` (varanda would have to
  share the hostname with vigia somehow — likely a sub-path; not
  worked out yet).
- `Settings` — when varanda grows a user preferences UI (light
  mode toggle, default landing view, …).
- `MemberDirectory` — when the family wants a roster view of who
  has access to what (read-only crachá viewer; admin would still
  edit via Lisp + git).

## 6. Add a new pleme-io product (not a service)

If pleme-io ships a new top-level product that should appear at
its own subdomain (e.g., `wiki.quero.cloud`, `chat.quero.cloud`):

1. **Add the Pages app** in `pages_apps[<product>].custom_domains`
   inside the cloudflare-pleme workspace. Don't add to varanda's
   list — it's a separate product, separate Pages project.
2. **Author the Helm chart / WASM bundle** for the product itself.
3. **Optionally** add a tile to varanda by labeling the product's
   HelmRelease with the saguão-service contract — it'll appear in
   the family's portal alongside the service tiles.

## Reference: where each expansion vector touches code

| Vector | Repos touched | Lines changed (typical) |
|---|---|---|
| Family member | `cracha/examples/family.lisp` | 1–5 |
| Service | `helmworks/charts/lareira-<svc>/` (one HelmRelease) + `cracha/examples/family.lisp` (grants) | ~20 + 1 |
| Cluster | `cracha/examples/<name>.lisp` (1 form) + `pangea-architectures/workspaces/cloudflare-pleme/domains/quero.cloud.yaml` (2 lines under pages_apps) + 4 cp operations from `cracha render cluster` | ~10 |
| Location | implicit in cluster | 0 (just the `:location` field) |
| View mode | `varanda/src/hostname.rs` + `varanda/src/view.rs` + maybe `varanda/public/industrial.css` | ~50 |
| New product | `pangea-architectures/workspaces/cloudflare-pleme/…` (1 pages_apps entry) + the product's own repo | ~10 + the product itself |

## Cross-references

- `theory/SAGUAO.md` §V — full cluster onboarding flow
- `theory/SAGUAO-MIGRATION.md` — phase-by-phase migration cookbook
- `blackmatter-pleme/skills/saguao/SKILL.md` — operational guide
- `cracha/CLAUDE.md` + `vigia/CLAUDE.md` + `passaporte/CLAUDE.md` — companion primitive docs
- `varanda/docs/design.md` — the aesthetic this expansion plays inside
