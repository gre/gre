---
title: 'Making performant React applications'
description: 'I would like to express here my opinion and feedback on using React and performance optimization you can do.'
thumbnail: '/images/2015/07/diaporama_3.jpg'
author: Gaetan
layout: post
tags:
 - react
 - vdom
---

*^ Sorry guys, you may have notice the blog post date is wrong. I won't change the URL, but thanks to how time works, this will be fixed in one month anyway :-D*

[ReactEurope](https://twitter.com/chantastic/status/616608931037646850) conference
was to me incredibly [inspiring](https://twitter.com/chantastic/status/616670658911715328) and [promising](https://twitter.com/chantastic/status/616995607043903488).
Yersterday got tons of news and tweets from JavaScript community.

One tweet and blog post by the great [@aerotwist](https://twitter.com/aerotwist) got my attention.

<blockquote class="twitter-tweet" data-cards="hidden" lang="fr"><p lang="en" dir="ltr">I often hear claims that “the DOM is slow!” and “React is fast!”, so I decided to put that to the test:&#10;&#10;<a href="https://t.co/M1RZZiyVT2">https://t.co/M1RZZiyVT2</a>&#10;&#10;🐢vs🐇</p>&mdash; Paul Lewis (@aerotwist) <a href="https://twitter.com/aerotwist/status/616934953679458304">3 Juillet 2015</a></blockquote>

I would like to express here my opinion and feedback on using React.

I've been using React for almost 2 years now, and always in performance intensive use-cases, from Games to WebGL.

<a href="http://diaporama.glsl.io/" target="_blank">
<img src="/images/2015/07/diaporama_3.jpg" alt="" class="thumbnail-left" />
</a>

I've created [glsl.io](http://glsl.io/) and I'm working on [Diaporama Maker](https://github.com/gre/diaporama-maker).
Both applications are built with React and combined use of HTML, SVG, WebGL.

Diaporama Maker is probably the most ambitious piece of software I've ever personally done.

<br style="clear: left" />

> In short, [Diaporama Maker](https://github.com/gre/diaporama-maker) it is a WYSIWYG editor for web slideshow (mainly photo slideshows). It is a bit like iMovie with the web as first target. [The project is entirely open-sourced.](https://github.com/gre/diaporama-maker)

Currently, I am able to render the whole application at 60 FPS and this is still unexpected and surprising to me
(press Space to run the diaporama on [diaporama.glsl.io demo](http://diaporama.glsl.io/)).
Well, more exactly, this would not have been possible without some optimizations
that I'm going to detail a bit at the end of this article.

<!--more-->

## The point is productivity

I don't think Virtual DOM claims to be faster than doing Vanilla DOM, and that's not really the point. **The point is productivity.**
You can write very well optimized code in Vanilla DOM but this might require **a lot of expertise**
and a lot of time even for an experienced team *(time that should be spent focusing on making your product)*.

When it comes to adding new features and refactoring old ones, this goes worse.
Without a well constrained framework or paradigm, things does not scale far, are time consuming and introduce bugs,...
Especially in a team where multiple developers have to work with each other.

> **See Also:** [Why does React scale?](https://www.youtube.com/watch?v=D-ioDiacTm8) by [@vjeux](https://twitter.com/Vjeux).

## What matters to me

There is a lot of advantages of using Virtual DOM approach before talking about React performances.

Of course, this always depends on what you are building, but I would claim that
**there is a long way to go using React before experiencing performance issues**,
and in the worse cases: **you can almost always find easy solutions to optimize these performance issues**.

### DX

React has an incredible Developer eXperience (that people seem to call DX nowadays!) that can [help you improving UX](https://twitter.com/greweb/status/617258379183005696) and the ability to [measure Performances](https://facebook.github.io/react/docs/perf.html) and [optimize them](https://facebook.github.io/react/docs/component-specs.html#updating-shouldcomponentupdate) when you reach bottlenecks.

You can easily figure out which component is a bottleneck in the Component tree as shown in following screenshot.

> ![](/images/2015/07/diaporama-perfs.png)
With printWasted() you can see how much time React has wasted to `render()` something that didn't change and how much instances has been created. (there is also printInclusive and printExclusive)

This is a bit equivalent of the Web Console Profiler except it emphasis on your application components which is a very relevant approach.

### React data flow

> I can't imagine re-writing Diaporama Maker in Vanilla DOM.

In Diaporama Maker, I have a lot of cross dependencies between components,
for instance the current `time` is shared and used everywhere in the application.
As a matter of fact, dependencies grow when adding more and more features.

> ![](/images/2015/07/diaporama_configure_kenburns.gif)
usages of time in 3 independent components.

**The descriptive Virtual DOM approach very simply solves this problem**.
You just have to pass props in to share data between components:
there is one source of trust that climb down your component tree via "props".

![](/images/2015/07/diaporama-maker-time-props.jpg)

With Virtual DOM approach, the cost to add one new dependency to a shared data is small and **does not become more complex as the application grows**.

> ![](/images/2015/07/diaporama_slide_content.gif)
another more complex showcase of shared states.

Using an Event System like you would do in standard Backbone approach tends to lead to imperative style and spaghetti codes (and when using global events, components are not really reusable).

Moreover, I think that `Views<->Models` Event System approach, if not carefully used, tends to converge to an unmaintainable and laggy applications.

### React is a Component library

React truly offers **component as first-class citizen**.
This means it allows component reusability. I've tried alternative like virtual-dom and I don't think it emphasizes enough on this benefit.

There are [important good practices](https://twitter.com/chantastic/status/616997918155759616) when using React like minimizing states and props and I'm not going to expand more on this subject. Most of these best practices are not exclusive to React but come from common sense and software architecture in general.
One of the important point for performance is to **choose a good granularity of your component tree**.
It is generally a good idea to split up a component into pieces as small as possible
because it allows to separate concerns, minimize props and consequently optimize rendering diff.

#### Diaporama Maker architecture

You would be surprised to know that Diaporama Maker does not even use **Flux** (that might be reconsidered soon for collaborative features). I've just taken the old "callback as props" approach all the way down the component tree. That easily makes all components purely modular and re-usable (no dependencies on some Stores).
I've also taken the [inline style approach]() without actually using any framework (this is just about props-passing `style` objects).

As a consequence, I've been able to externalize a lot of tiny components that are part of my application
so I can share them across apps and also in order to people to re-use them.

What is important about externalizing components is also the ability to test and optimize them independently (the whole idea of modularity).

Here are all the standalone UI components used by Diaporama Maker:

- [bezier-easing-editor](https://github.com/gre/bezier-easing-editor)
- [bezier-easing-picker](https://github.com/gre/bezier-easing-picker)
- [diaporama-react](https://github.com/glslio/diaporama-react)
- [glsl-transition-vignette](https://github.com/glslio/glsl-transition-vignette)
- [glsl-transition-vignette-grid](https://github.com/glslio/glsl-transition-vignette-grid)
- [glsl-uniforms-editor](https://github.com/gre/glsl-uniforms-editor)
- [kenburns-editor](https://github.com/gre/kenburns-editor)

(each one have standalone demos)


## Optimizing performances

<blockquote class="twitter-tweet" lang="fr"><p lang="en" dir="ltr">I&#39;ve been working on crazy projects using React (like <a href="http://t.co/U2oETh5lhZ">http://t.co/U2oETh5lhZ</a> ). most performance issues i&#39;ve met was not because of React</p>&mdash; Gaëtan Renaudeau (@greweb) <a href="https://twitter.com/greweb/status/617210444839809024">4 Juillet 2015</a></blockquote>
<script async src="//platform.twitter.com/widgets.js" charset="utf-8"></script>

Here are 2 examples of optimizations I had to do in Diaporama Maker that are not because of React:

- It is easy to write not very optimized WebGL, so I work a lot to optimize the pipeline of [Diaporama engine](https://github.com/gre/diaporama)
- CSS transforms defined on Library images was for a time very intensive for the browser to render so I am now using server-resized "thumbnails" instead of the full-size images. Asking the browser to recompute the `transform: scale(...)` of 50 high resolution images can be super costy. (without this optimization, the resize of the application was running at like 2-3 FPS because the library thumbnails need to recompute their scale and crop).

But what if you still have performance issue due by React? Yes this can happens.

### Timeline Grid example

In Diaporama Maker, I have a Component that generates a lot of elements (like 1300 elements for a 2 minutes slideshow) and my first naive implementation was very slow. This component is [TimelineGrid](https://github.com/gre/diaporama-maker/blob/b0c6447b127785bea3c2487b0c77037418298b8c/client/ui/TimelineGrid/index.js) which renders the timescale in the timeline. It is implemented with SVG and a lot of `<text>` and `<line>`.

The performance issue was noticeable when drag and dropping items across the application. React was forced to render() and compare the whole timescale grid every time. But the timescale does not change! it just have 3 props:

```xml
<TimelineGrid timeScale={timeScale} width={gridWidth} height={gridHeight} />
```

**So it was very easy to optimize it just by using the `PureRenderMixin` to say to react that all my props are immutable.**
(I could have implemented `shouldComponentUpdate` too).

After this step, and for this precise example, I don't think a Vanilla DOM implementation can reach better performance:

- when one of the grid parameter change, **EVERYTHING** need to be recomputed because all scales are changing.
- React is doing even smarter thing that I would not manually do? Like reusing elements instead of destroying/creating them.

There might still be ways to go more far in optimizing this example. For instance I could chunk my grid into pieces
and only render the pieces that are visible, like in an infinite scroll system *(I could use something like [sliding-window](https://github.com/gre/sliding-window) for this)*.
That would probably be premature optimization for this example.

## Wrap Up

To my mind, generic benchmarks always tends to be biased and does not represent use-cases reality unless you are really covering your application itself.

The TimelineGrid component optimization explained in this article is a very specific and well chosen example,
but it is one counter-example for such a benchmark.

Each application has its own needs and constraints and we can't really generalize one way to go.
Also Performance should not be the main concern to choose a technology.


It is easy to make Virtual DOM library benchmarks,
comparing the performance of rendering and Array diffing,
but does that covers 80% of use-cases?
Is performance really the point?
What tradeoff do you accept to make between Performance and Productivity?

Tell me what you think.

In the meantime, I think we can all continue getting applications done
and [developing amazing DX](https://github.com/gaearon/react-hot-loader).
