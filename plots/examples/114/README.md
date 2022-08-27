---
date: "2021-04-23"
title: "Growing parametric 14-20"
image: /images/plots/114.jpg
description: "stacking multiple parametric functions. Sakura Gelly Roll on black A4, revealing artifacts of the paper."
tags:
  - parametric
---

Another parametric functions. Inspired from [plot#111](/plots/111) idea: stacking multiple parametric functions. The parametric used is

```rust
let parametric = |t, p| {
  (
    (0.2 + 0.7 * p) * (2. * PI * t).sin()
        + 0.1 * (14. * PI * t).sin(),
    (0.2 + 0.8 * p) * (2. * PI * t).cos()
        + 0.2 * (20. * PI * t).cos(),
  )
};
```

Sakura Gelly Roll on black A4, revealing artifacts of the paper.
