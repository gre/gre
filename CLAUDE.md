# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

Monorepo for **greweb.me** — @greweb's generative plotter art portfolio. Combines a Next.js website with 1500+ Rust generative art examples, experimental doodle projects, shader experiments, and blockchain art.

## Common Commands

### Website (Next.js)

```bash
npm run dev          # Dev server (uses --openssl-legacy-provider for Node compat)
npm run build        # Production build
npm run start        # Start production server
```

### Plots (Rust generative art → SVG for pen plotters)

```bash
cd plots
cargo run --example 001                 # Run a specific plot, outputs image.svg
cargo run --example 001 -- --seed 42    # With specific seed (examples using clap)
cargo watch "run --example 001"         # Hot reload (watch for changes)
```

### Doodles (Rust/WASM generative art projects)

```bash
# From repo root, for a Rust-based doodle:
npm run build-rust-doodle               # Build Rust → WASM (uses scripts/build-rust-doodle.sh)
npm run build-doodle                    # Webpack bundle for web
npm run start-doodle                    # Parcel dev server
```

## Architecture

### Two main systems coexist:

1. **Next.js website** (`pages/`, `components/`, `posts/`) — portfolio site at greweb.me. Uses file-based routing, React 17, server-side rendering via `getStaticProps`.

2. **Rust generative art** (`plots/`) — daily art examples that output SVG files for physical pen plotting with AxiDraw. Each example is a standalone Rust binary.

### plots/ structure

- `plots/src/lib.rs` — shared utility library (`gre` crate): SVG helpers, math, color, image processing, TSP solver, etc.
- `plots/examples/NNNN/main.rs` — each numbered example is a standalone generative art piece
- `plots/examples/NNNN/README.md` — YAML frontmatter metadata (date, title, image, tags, NFT IDs)
- Examples use: `clap` for CLI args (seed, dimensions, pad), `svg` crate for output, `rand 0.5.6` for RNG, `f64` coordinates in millimeters

**Important**: `rand` version is **0.5.6** — use `rng.gen_range(min, max)` with commas, NOT `rng.gen_range(min..max)` with range syntax.

#### Typical plot example pattern:

```rust
use clap::*;
use gre::*;
use rand::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::*;

#[derive(Parser)]
struct Opts {
  #[clap(short, long, default_value = "image.svg")]
  file: String,
  #[clap(short, long, default_value = "210.0")]
  width: f64,                                        // A4 portrait
  #[clap(short, long, default_value = "297.0")]
  height: f64,
  #[clap(short, long, default_value = "10.0")]
  pad: f64,
  #[clap(short, long, default_value = "0.0")]
  seed: f64,
}

fn art(opts: &Opts) -> Document {
  // ... generate routes, build SVG using gre::* helpers ...
  // layer("0-name"), base_a4_portrait("white"), render_route(data, route)
}

fn main() {
  let opts = Opts::parse();
  let document = art(&opts);
  svg::save(opts.file.clone(), &document).unwrap();
}
```

Key `gre::*` helpers: `layer(id)`, `base_a4_portrait(bg)`/`base_a4_landscape(bg)`, `render_route(data, route)`, `mix()`, `smoothstep()`, `signature()`.

#### Plot README.md format:

```yaml
---
date: "YYYY-MM-DD"
title: "Plot Title"
image: /images/plots/NNNN.jpg
tags:
  - tag-name
---
```

Optional fields: `tweet`, `objkts` (NFT IDs), `sourceFolderURL`, `sourceFolder`, `noSource`.

### doodles/ structure

Standalone experimental projects with mixed tech stacks (Rust/WASM, WebGL, p5.js). Each has its own build system. Rust-based doodles follow the plottable template from `gre-plot-exploration-1`-style projects (Rust/WASM/WebGL with fxhash seeding, `f32` coordinates, custom palette system).

### Content pipeline

- **Blog posts**: `posts/YYYY-MM-DD-slug.md` → parsed by `posts/index.js` with gray-matter
- **Plots metadata**: `plots/examples/NNNN/README.md` → parsed by `plots/index.js` with gray-matter + webpack require.context
- **Shaderdays**: `shaderdays/` → manually imported in page components

### Coordinate conventions (plots)

- Units: millimeters. A4 = 210×297mm
- Origin: top-left, Y increases downward
- `pad`: margin (typically 5-10mm)
- SVG layers use `inkscape:groupmode="layer"` for Inkscape pen plotter workflow
