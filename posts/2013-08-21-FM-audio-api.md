---
title: 'Frequency Modulation (FM) with Web Audio API'
description: ''
author: Gaetan
layout: post
tags:
 - fm
 - audio
---

 [zoundarticle]: /2013/07/zound-live/
 [zoundrepo]: http://github.com/gre/zound-live/
 [zoundfm]: https://github.com/gre/zound-live/blob/master/app/assets/javascripts/modules/SimpleFM.js
 [fmwiki]: http://en.wikipedia.org/wiki/Frequency_modulation_synthesis

The main principle of [Frequency Modulation (FM)][fmwiki] is to **pipe an Oscillator (the Modulator)
into the frequency of another Oscillator (the Carrier)**.

This article will explain to you how FM Synthesis works with **interactive demos**.
In the meantime, all demos are implemented with the brand new **Web Audio API**,
so feel free to hack the code for your own purpose.

This article will also introduce some Audio concepts like **LFO**, **Envelope** and **Finetuning**.

I've recently implemented a very first FM in [ZOUND live][zoundarticle] - *a HTML5 collaborative audio tracker*,
giving much more powerful Synthesizers (see in the following video).

<iframe width="640" height="480" src="//www.youtube.com/embed/El4JvaDWQUM" frameborder="0" allowfullscreen></iframe>
[*(here is the implementation of that FM)*][zoundfm]


<!--more-->

## Dive into Frequency Modulation Synthesis

As mentioned previously, FM is about **piping an Oscillator (the <u>Modulator</u>) into the frequency of another Oscillator (the <u>Carrier</u>)**.

The Modulator oscillation only affects the oscillation frequency of the Carrier but is not directly an audio signal.

![](/images/2013/08/fm_principle.png)

The result of that modulation differs depending on each oscillator **frequency** and **amplitude**:

[![](/images/2013/08/Frequencymodulationdemo-td.png)](http://en.wikipedia.org/wiki/File:Frequencymodulationdemo-td.png)

***N.B.*** *Our interactive demos in this article will always play a sound and visualize it (waveform / spectrum analyzer).
You will have different kind of controls depending on each specific aspect I want to illustrate.*

*The demos should work on Chrome. __However if you get an AudioContext failure, please reload the page__ (you may not be able to start them all in one row).*

### LFO

**Low-Frequency Oscillation (LFO)** is very used in electronic music for making rythmic audio effects.

LFO is simply a specific subset of a oscillator in a sense that **its oscilation frequency is under
the human audible range (20 Hz)** and is then not generally used as an audio signal but as an effect controller.

For instance the frequency / the amplitude of an oscillator, or in the following example the frequency of the cut-off filter:

<audio src="http://upload.wikimedia.org/wikipedia/commons/e/e4/Lfo-cutoff-frequency-wobble-bass.ogg" controls></audio>

Now, as a first demo,
let's see what happens if our **FM Modulator is an LFO**,
*(i.e. if that Modulator is in low frequency range)*.

<iframe width="100%" height="310" src="http://fiddle.jshell.net/FvnJx/58/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/FvnJx/58/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>

Observe in the Carrier graphs how **the waveform is regulary compressed and decompressed**. If you increase the Modulator frequency, it will speed up this effect. A real FM is about speeding up that effect up to the audible range...

***N.B.*** *With _Web Audio API_ (more generally with any modular synthesizers) we can easily control any module parameter with an LFO:*

```javascript
lfo.connect(carrier.frequency);
```

#### Modulator in audible range

Now, if we increase the frequency to the hearing range, here is what happens:
*(in that example you can also change the Carrier frequency)*

<iframe width="100%" height="310" src="http://fiddle.jshell.net/x4CWR/36/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/x4CWR/36/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>


It's as if that **once the Modulator reaches that audible barrier, it kind of becomes a second audible synthesizer**,
even if it only modulate the frequency of the actual synthesizer.
However, it's completely different than playing the two synthesizers directly into the output,
again the modulator influence the frequency of the carrier and is not directly piped into the output audio signal.

*There is especially cool sound produced when the Modulator frequency is closed to the Carrier frequency. For more infos, see the <u>Finetuning</u> section.*


### Frequency ratios: harmonic or dissonant sounds

One thing you may also have notice in the previous example is that most of the generated sounds was quite dissonant, non harmonic.

Now, if we add more restrictions and only **snap the possible modulator frequencies**
to a **multiple of the carrier frequency**, here is what happens:

<iframe width="100%" height="310" src="http://fiddle.jshell.net/Euezv/17/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/Euezv/17/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>

This harmonic result is due a simple fact in music: **Mutiplying a note frequency by 2 is equivalent to Increasing that note by one octave,** meaning the note has the same tone but is one-octave higher. (and vice versa for the division). *BTW, you may have noticed that fact by repetition of peaks in the previous example Spectrum Visualization.*

Now we can release some restrictions by also allowing frequencies multiple of `carrier frequency / 4`, which means allowing to increase/decrease by an **octave**, a **semi-octave** or a **quarter-of-octave**.

<iframe width="100%" height="310" src="http://fiddle.jshell.net/DFSwN/13/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/DFSwN/13/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>


*Eventually you could even allow more freedom using multiple of `carrier freq / 12`, because an octave is equally divided by 12 in the [Chromatic scale](http://en.wikipedia.org/wiki/Chromatic_scale).*

### Mixing the power of the Modulator effect

A very interesting part of the job is also to change the **amplitude of the modulator**. So far, we used a full amplitude modulating the carrier frequency from 0 to 2-times its original frequency which produces a quite rough sound.

Try to change the modulator amplitude on the following demo:

<iframe width="100%" height="310" src="http://fiddle.jshell.net/DAT5S/12/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/DAT5S/12/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>

Technically, we can easily control that range to any by changing the gain of the Modulator with a `GainNode` which is just a tool to scale the amplitude of a signal.

### Envelope

Now, we need to add an **Envelope** for automating that amplitude change you just experiment with.

An envelope in electronic music will generally look like this:

[![](/images/2013/08/500px-ADSR_parameter.svg.png)](http://en.wikipedia.org/wiki/File:ADSR_parameter.svg)

An Envelope corresponds to a **note lifespan**.
It is the minimum required for making our Synth.

We will generally **automate that amplitude through time for each note triggered**.

Here is a demo.
Play, try to hold and release a note (using the Play button or SPACE), and observe how the Spectrum Analyzer is moving:

<iframe width="100%" height="400" src="http://fiddle.jshell.net/tyEKr/32/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/tyEKr/32/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>

**Two different envelopes** has been used: one for the **Modulator** and one for the **Carrier** which produce **different sound effects in a note lifespan**.

*We won't make an interactive demo for changing these envelope parameters,
but you can try them in the ZOUND project (or see again the video).*

### Finetuning

Another interesting effect occurs **when the frequency of the Modulator is very close to the frequency of the Carrier**.
In the following example, we have set both oscillators to the same frequency but we expose a "detune" parameter which allows to change a bit the frequency of the Modulator.

<iframe width="100%" height="400" src="http://fiddle.jshell.net/X95S6/10/show/light/" allowfullscreen="allowfullscreen" frameborder="0"></iframe>
<a href="http://jsfiddle.net/X95S6/10/" target="_blank" style="display: block; text-align: right">Open on jsfiddle</a>

You can slightly notice that a sound is regulary looping like if it was an LFO effect. You can also visualize it on the graph.

This effect corresponds to the **[phase](http://tinyurl.com/nzkus8) change between both oscillators**: it regulary change from **"in-phase"** state (where it have exactly the same sine waveform at the same time) to a desynchronize **"out-of-phase"** (because of the small detune), and then slightly go to the next "in-phase" step. More the frequencies are close, more it takes time to oscillate from phase to phase.

This effect is especially awesome when you start mixing multiple synths together and finetune a bit each one so they don't sound exactly on the same frequency.

### Modulating the Modulator

There is so much more possibilites to play with,
for instance, the previously introduced Envelope could be mixed
with an LFO to change the Modulator effect in a rythm,
but now let's see how we can...

**...modulate the modulator!**

Eventually we can make a stack of modulators and use different kind of waveforms
for more powerful effects:

![](/images/2013/08/fm_multiple.png)

> Be careful when playing with stack of modulators, it is quite easy to have saturated or noisy sounds.

As an example, I made this experiment which randomly takes different frequencies and amplitude for a stack of 5 modulators:

[**-> http://jsfiddle.net/s2MMR/45/ <-**](http://jsfiddle.net/s2MMR/45/)

Careful! this experiment is a bit crazy! but it shows how different patterns can be when playing with FM.

<!-- TODO soon...
## Last demo, polished FMs playing a famous song...

As a last demo example, and in a more readable & simple code, here is a polished example of FM.
-->

----

Also, **If you are interested by ZOUND live, [fork it on Github][zoundrepo].**
