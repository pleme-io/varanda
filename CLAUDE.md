# varanda — repo-level agent instructions

> Brazilian-Portuguese for "porch." This repo implements **varanda**,
> the family-facing PWA of saguão.

## Frame

- [`pleme-io/theory/SAGUAO.md`](https://github.com/pleme-io/theory/blob/main/SAGUAO.md) §III.4
- `blackmatter-pleme/skills/saguao/SKILL.md`
- `blackmatter-pleme/skills/cloudflare-headless-blog/SKILL.md` — sibling deploy pattern
- `blackmatter-pleme/skills/ishou/SKILL.md` — design tokens (NEVER hand-author colors / fonts / spacing)

## What this repo owns

- The Yew + WebAssembly single-page app
- Three view modes (fleet / location / cluster) keyed off the request hostname
- The crachá REST client (consuming `cracha-core` types via wasm-compatible serde)
- The passaporte JWT cookie reader
- The Cloudflare Pages deploy artifact

## What this repo does NOT own

- **Identity** — passaporte handles sign-in via redirect-to-Authentik. varanda only reads the cookie.
- **Authz decisions** — crachá owns "what can this user see." varanda only renders what crachá returned.
- **Protected user data** — the actual services (vault, photos, jellyfin, …) hold their own data. varanda only links to them.
- **Server-side logic.** Pure static SPA. If you need server state, the answer is "add it to crachá or its own service," not "make varanda dynamic."

## Conventions

- Yew via the substrate `wasm-build` helpers — never hand-author the `wasm-pack` invocation.
- Trunk for the local dev server + bundle.
- Design tokens consumed from ishou (CSS custom properties via `ishou-tokens.json`).
- No CSS files outside the ishou-rendered tokens — semantic class names only.
- One source of truth for service icons / labels: served by crachá REST API alongside the access manifest.

## Pillar 1 reminder

Rust + WASM. **No JavaScript** beyond the trunk-generated bootstrap.
**No CSS-in-JS.** **No framework** beyond Yew.
