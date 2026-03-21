# Agent Memory — Plottable Era / doodles monorepo

## Project Locations
- Medieval (reference): `/Users/gre/dev/plottables/doodles/plottable-era-medieval/`
- Industrial:           `/Users/gre/dev/plottables/doodles/plottable-era-industrial/`

## RNG & Seeding
- Crate: `rand = "0.8"`, `bs58 = "0.5"`
- Pattern: `rng_from_hash(&hash)` in `src/fxhash.rs` — decodes bs58, takes first 32 bytes as `StdRng::from_seed`
- Hash format: `"oo" + 49 base58 chars` (51 chars total)

## SVG Conventions
- ViewBox / units: millimeters. Standard output: 210×297mm (A4), pad=5mm
- No fills, stroke-only paths. Layers = one `<g>` per ink.
- `make_layers_from_routes_colors` groups `(usize, Vec<(f32,f32)>)` by color index.
- `regular_clip` clips routes against PaintMask before adding routes.
- RDP simplification applied in `lib.rs` after all routes collected: `rdp(&pts, precision)`.
- data-credits, data-hash, data-traits, data-palette, data-perf, data-effects-hot/water embedded as SVG attributes.

## PaintMask API (key methods)
- `paint_borders(pad)` — mark border exclusion zone
- `paint_columns_left_to_right(&|x| ridge_y..yhorizon)` — efficient column-range paint
- `paint_rectangle(minx, miny, maxx, maxy)` — fill rectangle
- `paint_circle(cx, cy, cr)` — fill circle
- `paint_polygon(poly)` — fill polygon
- `paint_polyline(route, strokew)` — paint stroke width around polyline
- `regular_clip(routes, paint)` — returns only unmasked portions
- NO `paint_point` method — use column sweep or paint_rectangle instead

## Module Structure (template for new Era projects)
```
src/
  lib.rs          — WASM entry point, render() fn
  global.rs       — GlobalCtx struct, Feature, rand_init constructor
  palette.rs      — Ink/Paper/Palette definitions + init()
  svgplot.rs      — make_document, make_layers_from_routes_colors, Ink, Paper types
  fxhash.rs       — rng_from_hash()
  effects.rs      — Effects struct, to_svg_metafields() (hot/water PNG base64)
  performance.rs  — PerfRecords (span/span_end timing)
  sandbox.rs      — debug scene (stub until objects exist)
  algo/           — fully reusable across projects
  objects/        — project-specific renderers
```

## Cargo.toml Dependencies (exact versions)
```toml
wasm-bindgen = { version = "0.2.88", features = ["serde-serialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
getrandom = { version = "0.2", features = ["js"] }
instant = { version = "0.1", features = ["wasm-bindgen"] }
rand = "0.8"
bs58 = "0.5"
fontdue = "0.8"
noise = "0.8"
image = "0.24"
base64 = "0.21"
[dev-dependencies]
rayon = "1.8"
```

## Common Pitfalls When Bootstrapping New Projects
- Stub modules with only `/** doc comment */` and no items → "expected item after doc comment". Use `//` instead.
- `renderable.rs` from medieval references `objects::army::human::Human` — must be removed.
- `sandbox.rs` from medieval has heavy medieval-specific imports — replace with a clean stub.

## Dev Loop
- `cargo run --example one` → generates `image.svg` in project root (font from `./static/PrinceValiant.ttf`)
- `cargo run --example many N` → generates N SVGs in `results/`
- `cargo run --example perf N` → benchmarks N seeds
- `cargo run --example stats N` → trait distribution across N seeds

## Critical Pattern: Back-to-Front Mountain Fill Clipping Bug
When rendering layered terrain back-to-front with PaintMask:
- Back mountains paint `ridge_y..height` into PaintMask
- Front mountain fill lines land INSIDE that painted area → they get clipped away by regular_clip
- **Fix**: snapshot PaintMask BEFORE any mountain body is painted; clip all mountain fills against that snapshot

```rust
let sky_paint = paint.clone(); // snapshot before any mountain paints
for layer in &layers {
    let out = regular_clip(&layer.routes, &sky_paint); // clip against sky only
    paint.paint_columns_left_to_right(|x| ridge_y..height); // paint mountain body
}
```

## Polygon Closure for paint_polygon
- `paint_polygon` requires the polygon to be closed (last point == first point)
- Always append the first vertex again at the end when building polygons for PaintMask

## Hatch Fill Lines Must Be Strictly Horizontal
- Pen plotter fills must use horizontal lines (constant y, variable x)
- Never use diagonal fills — they plot as diagonal strokes
- Generate: `routes.push((clr, vec![(x0, y), (x1, y)]));` at spacing ~0.7–1.6mm
