---
date: "2021-04-17"
title: "Perlin mountains"
image: /images/plots/108.jpg
objkts:
  - 36604
plotterfiles:
  - https://plotterfiles.com/profile/files/5e43a751-6fef-4abc-a92d-253fe5c890c8
tags:
  - perlin
  - mountains
---

<video src="/images/plots/108.mp4" width="100%" controls autoplay muted loop></video>

It took me 107 previous days of plotting to come up with an elegant idea: searching the best plot among many different variants can easily be done by working instead on a video of that plot! Then you can chose a frame to plot, it is both practical as well as you get a nice animation for free!

Some code snippet for video generation.

```bash
# generate all .svg in results, then:
cd results
mkdir pngs
for f in *.svg; do convert $f pngs/${f%.*}.png; done
ffmpeg -r 24 -i pngs/%d.png -pix_fmt yuv420p -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" out.mp4
```
