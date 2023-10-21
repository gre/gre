---
date: "2023-10-20"
title: "FROST (3)"
image: /images/plots/1283.jpg
sourceFolder: 1281
tags:
- A3
- inktober
- frost
---

This implements "A local cellular model for snow crystal growth" paper by Clifford A. Reiter (2004) in Rust language, and renders it as a physical pen plot.

This cellular automaton is so chaotic that the generator will run multiple simulations in parallel and select one to be chose.

I have refined the original cellular automaton with some local effects that will make the propagation varies (using a perlin noise) and will also spawn at random locations.

The rendering is using my "worms filling" technique that allows to fill the space with strokes, in a manner that also contributes to the frost visual effect.

I've been having so much fun exploring this algorithm, which isn't so complex but yield very interesting patterns.