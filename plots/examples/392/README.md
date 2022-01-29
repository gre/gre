---
title: "PI RNG"
thumbnail: /images/plots/392.jpg
---

This was implemented for [genuary.art](https://genuary.art) *"Create your own pseudo-random number generator and visually check the results."* of JAN.24. let me explain why:

**Each line is drawn from the bottom to the top.** Actually the line is splitted into many segments that are drawn one after the other following an angle. The angle is initially pointing up but on each iteration, it gets randomly offset by a random value. That's where the random number generator (RNG) happens: a value of 0.0 means (go left) a value of 0.5 keep moving forward and a value of 1.0 (go right).

The funny part about the RNG at stake here is that it's implemented like this:

```rust
let mut rng_v = i / count; // value from 0 to 1, unique for each line
let mut custom_rng = || {
    rng_v = rng_v + PI;
    rng_v % 1.0
};
```

this basically is just translating a value by PI and keeping the fractional part 😅

this is actually almost like a sawtooth signal with a small offset. **sawtooth that creates the oscillation effect on the line** and the small offset makes a lower frequency waving effect of the lines.

<img width="100%" src="https://user-images.githubusercontent.com/211411/151678315-f3f7a895-ecb3-4d8e-8bc5-9fc62255a33c.png">

**What i'm trying to prove here is that despite the very not random RNG, I still managed to get a result that appears to be random.**

was live coded and live plotted on https://twitch.tv/greweb
