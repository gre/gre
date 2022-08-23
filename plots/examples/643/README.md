---
title: "Kandinsky circle study"
image: /images/plots/643.jpg
tags:
  - wccchallenge
---

This experimental study is based on `Wassily Kandinsky, Circles in a Circle, 1923`. "Kandinsky" was a weekly theme by @sableraph's creativecoding weekly challenge.

I tried to infer the rules of this generator from Kandinsky work, implemented in Rust language, plotted with 5 inks: Amber, Soft Mint, Poppy Red, Skull & Roses and Black.

<img width="100%" src="/images/plots/643-alt.jpg"/>

As this is entirely made in generative art paradigm, I do not directly control the result but curated one among many outcomes.

The challenge of generative art is to get good results on all the outcomes, so you have to figure out what are the rule that consider the circles well organized. To achieved this, I organized the circles along a spiral from the center. The spiral is non linear (more stretched in the middle) to get a better concentration on the center. I enforced a padding to make sure there is a ~ constant space inside the main circle ring. The work is all made with 3 primitives: 
- **Circle** (stroked with circles, filled with spiral)
- **Stroke** (stroked with lines, possibly multiple line to get different stroke widths)
- **Ray** which gives this "spot light" effect of the yellow and turquoise polygons. They are filled with rays lines. The exterior line is doubled to create a bit of outline effect.

Here is a preview of other outcomes that could also have been plotted:

<video muted loop autoplay controls src="/images/plots/643-preview.mp4" width="100%"></video>

The generator was live coded on Twitch and a first prototype was plotted (with some failure on the first try 😂)

<video muted loop autoplay controls src="/images/plots/643-twitch.mp4" width="100%"></video>

The big challenge was to get the line distance right for the physical inks. It's very difficult because you get different ink flow from one ink to another.

From the live plotted failed try, I adjusted the line distance and changed pink into a dark blue and finally managed to get a nice outcome:

<video loop controls src="/images/plots/643-real-time.mp4" width="100%"></video>

<img width="100%" src="/images/plots/643z1.jpg"/>

<img width="100%" src="/images/plots/643z2.jpg"/>

<img width="100%" src="/images/plots/643z3.jpg"/>
