---
date: "2021-07-21"
title: "œil pour œil (4 frames)"
image: /images/plots/202.gif
description: "@greweb's #12 plot loop"
tags:
  - plotloop
---

@greweb's #12 plot loop. Taking the idea of [plot#198](/plots/198) to the next level using an animated eye.

These are plotted (~ 2 hours each frame) with fountain pens and two inks ('Bloody Brexit' by Diamine and 'Writers Blood' by Diamine) on a A4 watercolor paper (300g/m2).

The eye is mirrored with a different offset to make this duality. The title refers to the French expression "œil pour œil, dent pour dent" which means "an eye for an eye".

### Making of

I took a video of my wife eye and use this GIF as an input to my generator =)

<video src="/images/plots/202-input.mp4" width="100%" controls autoplay muted loop></video>

The way to mix the two eyes is using a simple technique, using my own tool set:

```rust
let get_color_1 = image_gif_get_color("images/eye.gif", opts.index).unwrap();
let get_color_2 = image_gif_get_color("images/eye.gif", opts.index + 2).unwrap();
let f = |(x, y): (f64, f64)| {
    let p = (x, y);
    let c1 = get_color_1((p.0, p.1 + 0.2));
    let c2 = get_color_2((p.0, 1.2 - p.1));
    smoothstep(0.0, 1.0, grayscale(c1)) *
    smoothstep(0.0, 1.0, grayscale(c2))
};
```
