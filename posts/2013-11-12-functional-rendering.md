---
title: 'Functional Rendering'
description: "This talk explains how to think the rendering in a more functional way. Let's see how we can just do things with a function of (Vec2 => Color)."
author: Gaetan
layout: post
tags:
 - functional
 - rendering
 - GLSL
---

[Gamelier]: http://gamelier.org/gaetan-renaudeau-on-procedural-vs-functional-rendering/
[slides]: http://greweb.me/prez-functional-rendering

I've done a talk at **[Gamelier][Gamelier]** last Monday about 
how to think the **rendering** in a more **functional** way.

## Abstract

Most of today 2D graphics libraries restrict us to a set of primitive procedures (`drawRect`, `drawCircle`, `drawImage`,...) but when it comes to bring more interesting features you tends to be stuck with it. Let's see how we can just do things with a function of `(Vec2 => Color)`.

This is the way (*WebGL*) **GLSL** has already took and the presentation examples will be built on it.
Let's see what are the multiple benefits of taking that paradigm of rendering.

## Talk

<iframe src="//player.vimeo.com/video/78804695?title=0&amp;byline=0&amp;portrait=0" width="600" height="337" frameborder="0" webkitallowfullscreen mozallowfullscreen allowfullscreen></iframe>

### [Open the slides][slides]

## Checkout more presentations at Gamelier.org

[![](/images/2013/11/gamelier.png)][Gamelier]
