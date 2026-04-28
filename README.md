# varanda — family-facing PWA for the saguão fleet

> Brazilian-Portuguese for "porch / front balcony." Where guests are
> first received when arriving at the house.

`varanda` is the **user-facing portal** of the saguão fleet identity
+ authz + portal architecture. It's a Yew + WebAssembly single-page
app served from Cloudflare Pages at the apex of `quero.cloud` and
every `<location>.quero.cloud` and `<cluster>.<location>.quero.cloud`.

**Canonical architecture:** [`pleme-io/theory/SAGUAO.md`](https://github.com/pleme-io/theory/blob/main/SAGUAO.md) §III.4.

**Status:** scaffold. **Phase 6** of the saguão migration. Not yet
deployed.

## What it does

Three view modes (one bundle, hostname-keyed):

| View mode | Hostname | Renders |
|---|---|---|
| **Fleet** | `quero.cloud` | All locations the user can see, all clusters in each, all services in each cluster |
| **Location** | `bristol.quero.cloud`, `parnamirim.quero.cloud`, ... | Just that location's clusters + services |
| **Cluster** | `rio.bristol.quero.cloud`, `mar.parnamirim.quero.cloud`, ... | Just that cluster's services |

For each: read the passaporte JWT cookie set by Authentik on
sign-in; query the crachá REST API
(`GET /accessible-services?user=<sub>`) for the user's portal
manifest; render only the tiles the user is allowed to see.

Clicking a tile deep-links to the actual gated service (e.g.,
`vault.rio.bristol.quero.cloud`); Cloudflare routes the request
into rio's ingress-nginx; vigia gates the request via forward-auth.

## Why Cloudflare Pages

- **Freescape.** Pages is free and unlimited.
- **Static-only.** varanda has zero server-side state — no
  secrets, no API logic of its own. Pure HTML+JS+CSS+WASM
  bundle.
- **Same anycast edge as Cloudflare Tunnel.** Latency is uniform
  with the gated services it links to.
- **Wildcard custom domains.** One Pages project handles
  `quero.cloud` + every subdomain via wildcard routing.

## Why Yew

- **Rust end-to-end.** Shares `cracha-core::AccessPolicy` types
  with crachá; the API response deserializes directly into Rust
  structs.
- **Shared JWT verification helpers** with passaporte (kenshou-based).
- **Aggressive tree-shaking** (~150KB gzipped target).
- **Substrate ships `wasm-build`** helpers used by other Yew apps in
  the fleet (compass-nvim, etc.).

## Repo layout

```
varanda/
├── README.md
├── CLAUDE.md
├── flake.nix                       (substrate wasm-build)
├── Cargo.toml
├── Trunk.toml                      (trunk-rs build config)
├── index.html                      (PWA shell)
├── src/
│   ├── main.rs                     (Yew app entry)
│   ├── view/
│   │   ├── fleet.rs                (apex quero.cloud view)
│   │   ├── location.rs             (<location>.quero.cloud view)
│   │   └── cluster.rs              (<cluster>.<location>.quero.cloud view)
│   ├── api.rs                      (crachá REST client)
│   └── session.rs                  (passaporte JWT cookie reader)
├── public/                         (static assets — favicons, manifest.json)
└── ishou-tokens.json               (consumed design tokens)
```

## Bootstrap

```bash
nix develop
cargo generate-lockfile
trunk serve                          # local dev server
trunk build --release                # production bundle in dist/
```

Deploy via Cloudflare Pages (Wrangler):

```bash
nix run .#deploy-pages
```

## Cross-references

- [`SAGUAO.md` §III.4](https://github.com/pleme-io/theory/blob/main/SAGUAO.md)
- `blackmatter-pleme/skills/saguao/SKILL.md`
- `blackmatter-pleme/skills/cloudflare-headless-blog` — sibling pattern (zuihitsu blog) for the deploy machinery
- Companion repos: `pleme-io/passaporte`, `pleme-io/cracha`, `pleme-io/vigia`

## License

MIT.
