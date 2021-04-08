---
title: 'Introducing gl-react'
description: 'This presentation is an introduction to WebGL in React and functional programming concept and showcases made with gl-react and gl-react-native'
thumbnail: '/images/2015/10/funrendering.png'
author: Gaetan
layout: post
tags:
 - gl-react
---

<img src="/images/2015/10/gl-react.png" alt="" class="thumbnail-left" /> Last Thursday, my talk at [React Paris Meetup](http://www.meetup.com/ReactJS-Paris/events/226103821/) was about using **the functional rendering** paradigm of **WebGL** in **React**. The library [`gl-react`](https://github.com/ProjectSeptemberInc/gl-react) wraps WebGL in React paradigm with a focus for developing 2D effects, that we need in my current startup, [Project September](https://twitter.com/ProjSeptEng), where I have the chance to develop it.

## [Slides](http://greweb.me/reactmeetup7)

<iframe src="http://greweb.me/reactmeetup7" width="600" height="400" frameborder="0"></iframe>

<!--more-->

## Abstract

We can write effects without having to learn the complex and imperative low-level WebGL API but instead composing React components, as simple as functional composition, using VDOM descriptive paradigm.

[gl-react](https://github.com/ProjectSeptemberInc/gl-react) brings WebGL bindings for react to implement complex effects over content.

[gl-react-native](https://github.com/ProjectSeptemberInc/gl-react-native) is the React Native implementation,
therefore allows universal effects to be written both for web and native.

These libraries totally hides for you the complexity of using the OpenGL/WebGL API but takes the best part of it: GLSL, which is a "functional rendering" language that runs on GPU.
