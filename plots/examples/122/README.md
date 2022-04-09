---
title: "Growing parametric splitted"
image: /images/plots/122.jpg
description: "stacking multiple parametric functions with 3 stops. playing with moiré effects. STA pigment liner on A4 bristol."
tags:
  - parametric
---

stacking multiple parametric functions with 3 stops. playing with moiré effects. STA pigment liner on A4 bristol.

```rust
let size = 90.;
let f1 = (8., 8.);
let f2 = (5., 40.);
let amp1 = 1.0;
let amp2 = 0.05;
let samples = 100000;
let spins = 200.0;
let splits = 4.0;

let parametric = |p: f64| {
  let p1 = (splits * p).floor();
  let p2 = splits * p - p1;
  let t = (p1 + 0.8 * p2) / splits;
  let scale = 1.0 - t;
  let mut p = (
    scale
    * amp1
    * ((spins * 2. * PI * t).cos()
      + amp2
      * mix(
        (spins * f1.0 * PI * t).cos(),
        (spins * f2.0 * PI * t).cos(),
        t,
      )),
    scale
    * amp1
    * ((spins * 2. * PI * t).sin()
      + amp2
      * mix(
        (spins * f1.1 * PI * t).sin(),
        (spins * f2.1 * PI * t).sin(),
        t,
      )),
  );
  let noise_angle = 2.
    * PI
    * perlin.get([
      0.02 * p.0,
      0.02 * p.1,
      100.0 + opts.seed,
    ]);
  let noise_amp = 0.1
    * perlin.get([
      0.01 * p.0,
      0.01 * p.1,
      opts.seed,
    ]);
  p.0 += noise_amp * noise_angle.cos();
  p.1 += noise_amp * noise_angle.sin();
  p
};
```
