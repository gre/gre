---
date: "2021-04-25"
title: "Triangle planet"
image: /images/plots/116.jpg
description: "Exploration of perlin noise triangle planets. various fountain pen on A4 bristol."
tags:
  - parametric
  - perlin
  - planet
---

**Exploration of perlin noise triangle planets. various fountain pen on A4 bristol.**

These are taking almost one hour to plot, with 2 passes of 2 different spirals. Each spiral varies a bit with different offset and displacement noise which creates a desired effect of bolder areas, caveats and craters.

The displacement noise is a simple perlin noise, only one harmony is used but the noise is split into two 2 different perlin noises: the angle noise (like a vector field, different angle of displacement) and the amplitude noise (how much does the displacement move the point).

The triangle is made with a parametric function, reusing technique explored on [plot#114](/plots/114).
There is an important effort on finding a good balance between noise and distribution of lines (it's not linear, it's closer on edges).
