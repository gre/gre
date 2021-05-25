---
title: "Rainbow Roots"
thumbnail: /images/plots/143.gif
description: "8 frames plotted making an animated loop. A 1920p video and A4 physical art is available as an NFT."
tags:
  - field
  - perlin
  - plotloop
objkts:
  - 98062
---

Here is "Rainbow Roots", my 4th [**plot loop** (see article)](https://greweb.me/2021/05/plot-loops). **The main digital art is a 1920p video loop of 8 frames available as a [Tezos hicetnunc NFT](https://www.hicetnunc.xyz/objkt/98062)**. The physical art are the 8 frames, plotted with 2 fountain pens on Bristol A4 paper (250g), and offered when [buying the NFT](https://www.hicetnunc.xyz/objkt/98062).

There are 8 plots available for sale and there will be no other editions of these plot loop frames. You can chose which frame you buy as the NFT editions are segmented into different prices. 20.1 tez for the first frame, 20.2 for the second and so on up to 20.8.

<img src="/images/plots/143-plots.jpg" width="100%">

This is a reboot of [plot#091](/plots/091) with more lines and better noise technique using domain warping.

### Coloring

Each plotted frame is made with 2 fountain pens, one primary and one secondary ink. The primary color is interleaved with the second with a ratio of 2/3 for primary and 1/3 for secondary.

I used 8 different inks, all by "Diamine" brand: Turquoise, Aurora Borealis, Bloody Brexit, Imperial Purple, Writer's Blood, Red Dragon, Pumpkin, Sepia.

<img src="/images/plots/143-inks.jpg" width="100%">

I had a last minute changes after the photo was taken: I replaced Spring Green by another color because it would have been too "light" for a plot. I actually tried to do one with Sepia and it was also too "light", I decided to go black! I definitely have betterplans for 'Spring Green'.

### Paper

I use a Canson Bristol 250g paper which works great with fountain pen. Format A4.

<img src="/images/plots/143-zoom1.jpg" width="50%"><img src="/images/plots/143-zoom2.jpg" width="50%">

## Creation process

The creation process is made of many steps, which are entirely created by me and all these steps are fully published and open sourced.

- art generator: I write a Rustlang program that generate SVG files. (see _main.rs_)
- GIF preview: I use a script to make a digital and theorical video of the animation. Very important for me to have an idea of the animation (even tho only plotting time will have the final surprise).
- plot first prototypes: I do some prototype plots to make sure the plot is good, specifically that the density chosen (number of lines) is well adjusted (too much and your paper starts to be comprised, not enough and you have too much gaps).
- I polish a lot the generator. Specifically on the different noise harmonies, frequencies and amplitudes.
- Once it's ready, i'll run a super script that loops over the "video preview" generation. It's time consuming as I will often stop and polish again the generator. This time it took me probably ten times to iterate like this. Literally the whole day.
- When I'm confident, I'll generate a lot of video previews. **This time I have generated 500 GIFs.** It was very tricky to decide and to actually elect the final plot, I developed my own tool (see section below)
- Finally, I can plot them all, it takes a lot of caution on manipulating paper and fountain pens and a lot of manual actions. It is very time consuming but very rewarding. **Each frame took more than an hour to plot with an AxiDraw robot.**

### Preview

This is what the theorical art was going to be, this is a digital preview before doing the plot so indeed it only simulate what the actual ink was going to do. The physical art is better looking with the imperfections of the medium.

<img src="/images/plots/143-theorical.gif" width="100%">

### Process to find the final plot

As I'm working with a generator, it can generate infinite variants of plot loops. I have generated 500 GIFs (virtual plot loops)

It ended up being very tedious to try to find a good animation by manually going through the files so I literally developped an app to solve my problem and provide a voting system so I can compare in parallel different results and chose among them. It could be reused and improved in future, it's not yet "generic" but if there is a need for it, I would make it generic and open source it as a standalone tool.

<img src="/images/plots/143-elector.jpg" width="100%">

### Early prototype and Special Editions

This prototype was done to adjust some plotting parameters, specifically the number of lines.

<img src="/images/plots/143-prototype.jpg" width="100%">

I've also made these 2 special editions one to offer a friend and one gift to buyer of one of my previous plot loop. Each of them have very specific variations of parameters (they are written on the back of the plot and i don't keep them in memory, basically they are technically not easy to reproduce identically!)

<img src="/images/plots/143-special1.jpg" width="100%">
<img src="/images/plots/143-special2.jpg" width="100%">
