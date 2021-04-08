---
title: "ZOUND live project initiated"
description: 'Last week, I initiated ZOUND live following my previous "ZOUND" experiment but being much more ambitious this time: using both the Audio API, the new MIDI API and electronic music software experience, we start our own web collaborative audio modular tracker.'
thumbnail: /images/2013/07/nanokontrol.jpg
author: Gaetan
layout: post
tags:
  - MIDI
  - audio
  - hackday
  - zound
---

[zound]: /2012/08/zound-a-playframework-2-audio-streaming-experiment-using-iteratees/
[webmidiapi]: http://webaudio.github.io/web-midi-api/
[webaudioapi]: https://dvcs.w3.org/hg/audio/raw-file/tip/webaudio/specification.html
[tracker]: http://en.wikipedia.org/wiki/Tracker_(music_software)
[zenexity]: http://zenexity.com

<img src="/images/2013/07/nanokontrol.jpg" alt="" class="thumbnail-left" />

Last week, I initiated, with my [Zenexity][zenexity] Hackday team, **"ZOUND live"**
following the previous ["ZOUND"][zound] experiment but being much more ambitious this time:
using both the **Audio API**, the _new_ **MIDI API** and electronic music software experience,
we start our own **web collaborative audio modular tracker**.

### Live demo of the Hackday application

<iframe width="640" height="480" src="//www.youtube.com/embed/uyHWhCnE4L0" frameborder="0" allowfullscreen></iframe>

<!--more-->

## Inspiration

A lot of features have been inspired from existing software like _SunVox_ or _Renoise_.
However, our version uses 100% web technologies and add collaborative and real time aspects.

<img src="/images/2013/07/sunvox.png" style="max-width: 300px" />

### Our Tracker

The application has a [tracker][tracker] where you can put notes.

<img src="/images/2013/07/tracker.png" style="max-width: 300px" />

### Our Audio modules

The application integrates a [modular music](http://en.wikipedia.org/wiki/Modular_software_music_studio) concepts.

<img src="/images/2013/07/nodeeditor.png" />

## The web techs

### About Web MIDI API

We bought a few **cheap MIDI controllers to interact with our application**.

<img src="/images/2013/07/midicontrollers.jpg" class="thumbnail-right" style="max-width: 250px" alt="" />

MIDI means _Musical Instrument Digital Interface_,
it is the **protocol** used by a lot of electronic musical instruments for a few decades.

The [Web MIDI API](webmidiapi) is a recent specification which makes MIDI devices accessible from a web page,
via a Javascript API.

Recently, _Chrome_ has started to [implement it](https://code.google.com/p/chromium/issues/detail?id=163795)
and it is available under _Chrome Canary_ (the dev version) via a _flag_ that you need to enable.

This is the perfect time to start experimenting it!

However, what I feared the most happened on the Hackday: **the MIDI API was broken on the morning
after a Chrome update during the night!** A first version of a browser MIDI permission was implemented
but I never succeeded to make it working. The state of the API seems to be still broken on Mac as of writing.

<blockquote class="twitter-tweet"><p><a href="https://twitter.com/greweb">@greweb</a> If you know how to build chromium, I may be able to provide a patch to enable it. But it isn't so long until Canary supports it.</p>&mdash; とよしま (@toyoshim) <a href="https://twitter.com/toyoshim/statuses/360685543778041857">July 26, 2013</a></blockquote>
<script async src="//platform.twitter.com/widgets.js" charset="utf-8"></script>

Well, that was already too late for the Hackday,
Fortunately we fallbacked on an alternative which relies on a Java applet to access MIDI devices, it was a laggy polyfill though...

**Lesson learned:** a nightly feature is a nightly feature, never assume features you add via flags are stable _(I never did, but it was a Hackday afterall!)_.

BTW, cheers to <a href="https://twitter.com/toyoshim">@toyoshim</a> who is implementing the MIDI API in Chrome :-)

### Using Web Audio API

The [Web Audio API][webaudioapi] is _a high-level JavaScript API for processing and synthesizing audio in web applications_.

The good thing about this API: it is already an **modular audio API**, so it's not so hard to build a modular audio application on top of it!

### Playframework

[Playframework](http://playframework.com/) has been used for **broadcasting events
between clients via WebSocket and synchronize everything on the interface**.
It is only broadcasting and does not save the song yet.

### Backbone.js

[Backbonejs](backbonejs.org) was used for the **models**, **views** and its nice **event system**.
It was a good library for prototyping and architecture the different parts of the application.

I found Backbone.js especially good when linking all parts together and especially for the network logic.
This leads to a very reactive style of programming:

<script src="https://gist.github.com/gre/6107277.js"></script>

## The team

**This project has been started during our monthly Hackday at [Zenexity][zenexity],
I want to thank my 6 awesome coworkers for being part of the project:**

- [@mrspeaker](http://twitter.com/mrspeaker) for his awesome electronic music knowledge.
- [@bobylito](http://twitter.com/bobylito) for his brilliant ideas and his JavaScript skills.
- [@mandubian](http://twitter.com/mandubian) for his playframework experience and JSON superpower!
- [@etaty](http://twitter.com/etaty) for helping with the server synchronization.
- [@skaalf](http://twitter.com/skaalf) for his cool DrumBox module.
- [@Noxdzine](http://twitter.com/Noxdzine) for his talentuous design.

This was actually my first real project managment and it was quite cool!

**Hackday is only one day** and such an ambitious project is hard to achieve one in a row,
the project architecture needed to be a bit ready and having a PoC working before the Hackday. Also I wanted everyone to have fun by experimenting with the Audio API parts and not to be blocked on boring parts.

As a team manager, I also had to define goals to achieve for the Hackday.

Woo, I realize that's **not an easy task to manage a team when running out of time!**

But fortunately, I think we fulfilled it just in time!

We ended the Hackday with a **Real Time demonstration of our application** with 4 people interacting together
with MIDI controllers.

## More to come!

Today, we have a **first working version of a
collaborative tracker with basic modular audio features**:

- MIDI note support + MIDI control assignation allowing to change module properties.
- a unique tracker with a 32 lines loop and 23 tracks.
- Synchronisation of everything: the tracker and modules for all connected clients.
- off-mode allowing one user to prepare a track which is muted for other users.
- play/pause and record mode!
- cursor of users displayed on the tracker.

**Stay tuned because there is so much features to come!**

[**The project on Github**](http://github.com/gre/zound-live)
