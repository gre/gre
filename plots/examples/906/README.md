---
date: "2023-02-12"
title: "Gold & Silver squares"
image: /images/plots/906.jpg
nft: https://objkt.com/asset/KT1FxuHijMyjUiHbAd9b3faaRUvo95oDnjM5/1
tweet: https://twitter.com/greweb/status/1624738454717321217
tags:
- A4
- wccchallenge
---

This physical plot was made with gold and silver gel pens on A4 Fabriano Black Black paper.

Squares are organized with their centers aligned on a grid but with size, color and filling that varies based on Perlin noise. This 1:1 was created during &SableRaph's WCC challenge with "Noise" prompt.

Clipping is used to cut lines that are hidden by a square already (only strokes are done as it's a plot, can't cheat with filling black).

**I took this opportunity to also sell this physical in an auction where the sale funds will be redirected to fundraise for TezQuakeAid. https://objkt.com/auction/e/nDdwPBv5** 

---

After 2 years of plotting, I finally figured out how to do proper clipping in my code. 🤣

I had a bug in the past where I was only using a dichotomic search but no subdivisions and stepping in order to correctly find collisions. Finally figured it out and this utility function will be useful in the future.

See source code link above for more details.

```rust
fn clip_routes(
  input_routes: &Vec<(usize, Vec<(f64, f64)>)>,
  is_outside: &dyn Fn((f64, f64)) -> bool,
  stepping: f64,
  dichotomic_iterations: usize,
) -> Vec<(usize, Vec<(f64, f64)>)>
```