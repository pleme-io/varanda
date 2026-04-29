# varanda — design language

> **Aesthetic in one line:** ink-black on paper-white, mechanical
> industrial, bold, shadowy. The pleme-io swerve mark is the only
> ornament.

## Sources of truth

| Concern | Where it lives | Never hand-author |
|---|---|---|
| Color (palette + semantic roles) | [`ishou`](https://github.com/pleme-io/ishou)'s `ColorPalette` + `SemanticRoles::pleme_dark()` | hex codes anywhere in this repo |
| Typography | ishou's `Typography` (serif / sans / mono / display) | font stacks |
| Spacing | ishou's 13-step 4px scale | `px` / `rem` literals |
| Shadow | ishou's `Shadow::brand_bold` (the signature drop) | manual `box-shadow` values |
| Brand mark | ishou's `Brand::pleme().swerve` | inline SVG paths (cite ishou) |

If you need a value that isn't in ishou yet, **add it to ishou
first**, regenerate the tokens, then consume. The single source of
truth keeps every pleme-io product visually coherent at zero
coordination cost.

## The aesthetic — five rules

1. **Two colors, used loud.** Ink (`#0A0A0A`) and paper
   (`#F5F5F0`). Anything else is wrong. Subtle scanline texture
   (4px alternating bands) is allowed — it's mechanical, not
   colorful.
2. **Hard edges, no gradients.** All borders are 1–2px solid
   `currentColor`. No `border-radius` greater than 0 anywhere
   except where ishou explicitly says so.
3. **The brand-bold drop is the signature.** Every interactive
   surface (tile, card, button) carries a hard offset shadow:
   `box-shadow: 6px 8px 0 0 currentColor`. The shadow is opaque
   and shifts on hover.
4. **Monospace is for emphasis.** Display headings + status
   labels + hostnames use the mono stack. Body copy is sans.
   Serif is reserved for long-form prose (currently unused).
5. **The swerve is rare.** The pleme-io swerve mark appears once
   per page — in the header — at 36px. Never as a background, never
   tiled, never tinted.

## Typography scale

From ishou's `Typography::scale`:

| Token | rem | px @ 16px root |
|---|---|---|
| `xs` | 0.75 | 12 |
| `sm` | 0.875 | 14 |
| `base` | 1 | 16 |
| `md` | 1.125 | 18 |
| `lg` | 1.25 | 20 |
| `xl` | 1.5 | 24 |
| `2xl` | 2 | 32 |
| `3xl` | 2.5 | 40 |
| `4xl` | 3 | 48 |

varanda uses: `xs` (host pills, dashed-rule labels), `sm` (descriptions),
`base` (body), `lg` (tile names), `2xl` (header h1).

## Color roles in use

From ishou's `SemanticRoles::pleme_dark()` (auto-mapped to palette
keys via the rendered `--ishou-color-*` CSS vars):

- **`background`** → polar-night-0 (default body bg)
- **`text`** → snow-storm-2 (default body fg)
- **`ink`** / **`paper`** — the brand monochromes; varanda uses
  these directly because the aesthetic IS bold ink-on-paper or
  paper-on-ink, not a Nord variant. The `industrial.css` layer
  binds `body { color: var(--ishou-paper); background: var(--ishou-ink) }`
  by default, with `.light-mode` flipping them.

The Nord palette stays available for future themed cluster portals
(e.g., a "frost" theme for a cold-storage cluster) without needing
to add tokens.

## Layout primitives

- **`.varanda-root`** — max-width 1280px, padded by `space-12`/`space-6`.
- **`.varanda-header`** — flex row: swerve mark (left), title
  (center-ish), user pill (right). 2px hairline rule below.
- **`.varanda-group`** — section per location/cluster pair. Dashed
  hairline rule under the section title.
- **`.varanda-tile-grid`** — auto-fill grid, 240px min column
  width, `space-6` gap.
- **`.varanda-tile`** — the punched-card panel. 2px border, no
  radius, brand-bold shadow, 80ms hover lift.

## Light mode

`<body class="light-mode">` flips ink↔paper. The scanline texture
swaps direction (subtle dark bands on paper). Toggle UI is not yet
wired (Phase 7 of saguão); the class hook is in place for when
varanda gains a settings drawer.

## Accessibility

- `prefers-reduced-motion` disables the tile lift transition.
- `:focus-visible` on tiles emits a 3px outline at +4px offset.
- Color contrast ratio of paper-on-ink: 17.5:1 (WCAG AAA).
- Tap targets: tiles are minimum 240×120px.

## Anti-patterns

| Don't | Do |
|---|---|
| `style="color: #fff"` inline | use ishou tokens via class |
| `border-radius: 4px` | radius 0 (or token if needed) |
| `box-shadow: 0 4px 20px rgba(0,0,0,0.5)` | the brand-bold drop only |
| Custom favicons / brand assets | consume ishou's `Brand::pleme()` |
| Importing Google Fonts | use ishou's typography stacks (system + Inter for sans, JetBrains Mono for mono) |
| Animations longer than 120ms | snap is the aesthetic — keep transitions tight |
| Color other than ink/paper for primary surfaces | full ink/paper monochrome IS the brand |

## Adding a new view mode

When a new `ViewMode` lands in `src/hostname.rs` (e.g., a future
`ServiceDetail { app, cluster, location }`):

1. Add the variant + a hostname recognizer test.
2. Add a Yew component under `src/view.rs` (or split into
   `src/view/<mode>.rs` once views grow).
3. Re-use the existing `varanda-root` / `varanda-header` /
   `varanda-tile-grid` primitives — don't introduce new layout
   shapes unless the existing ones genuinely don't fit.
4. If you need a new layout shape, add the relevant CSS class to
   `industrial.css` (consuming ishou tokens only) — never inline.

## Adding a new aesthetic primitive

If a tile shape that isn't in `industrial.css` recurs, add it as a
new class with the `varanda-` prefix. If a token shape that isn't
in ishou recurs, **add it to ishou** and regenerate. Never bypass
ishou.
