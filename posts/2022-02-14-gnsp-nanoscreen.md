---
title: "GNSP – the Nano screen rendering"
thumbnail: /images/2022/gnsp/screen-thumbnail.png
description: "This third article reveals the technique used to render the screen display itself."
tags:
  - NFT
---


This third article (in a series of 7 articles) reveals the technique used to render the screen display itself.

<video muted loop autoplay controls src="/images/2021/12/gnsp/419.mp4" width="50%" style="float:left; margin-right: 40px; margin-bottom:20px"></video>

**Timeline:**

- [article 1: GNSP – the concept](/2021/12/gnsp)
- [article 2: the 3D distance to a Nano S+](/2021/12/gnsp-raymarching)
- [**article 3: the nano screen**](/2022/02/nanoscreen)
- article 4: the swivel
- article 5: the background
- article 6: the video generation
- article 7: the final drop
- (Mid-January) public mint

**The collection is browsable on https://greweb.me/gnsp**

**OpenSea: https://opensea.io/collection/gnsp**

<br style="clear:left"/>

The screen, displays the unique BIP39 word and can sometimes have an effect or an animation. In the NFT metadata, they are expressed on the "Screen" feature, and here is the distribution:

```
                 <not defined>: 1527 = 74.6%
                     scrolling: 251 = 12.3%
                      blinking: 136 = 6.6%
                      negative: 81 = 4.0%
                       complex: 25 = 1.2%
        negative and scrolling: 11 = 0.5%
                 half-negative: 8 = 0.4%
         negative and blinking: 7 = 0.3%
    half-negative and blinking: 2 = 0.1%
```

This means 75% of the time you will only get the text displayed statically but in other case you have various effects implemented.

## Step 1: a Canvas2D texture is used for the word text

a 128 by 64 Canvas 2D texture is generated – this is the actual resolution in pixels on the actual device.

```js
function screen(word) {
  const w = 128; const h = 64;
  const canvas = document.createElement("canvas");
  canvas.width = w; canvas.height = h;
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "#fff";
  ctx.fillRect(0, 0, w, h);
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.font =
    (navigator.userAgent.includes("Mac OS")
      ? "" : "bold ") + "22px Arial";
  ctx.fillStyle = "#000";
  ctx.fillText(word, w / 2, h / 2);
  return canvas;
}
```

> Ok, there is a funny trick here on an annoying fact: depending on your OS you will have a different font weight, as Mac devices tend to have bolder font, I only used non bold in other cases. 


<!--
I will explain in Article 6 why I actually needed to support different OSs during the video generation phase. I have also externalized the `document.createElement("canvas")` and `ctx.fillText` implementations into a function in order to be able to run this in Node.js (against node-canvas library).
-->

Apart this trick, there are nothing fancy here: we are just writing the word in the Canvas. So there are actually no animation logic at all here: the animations (text motion / balls motion in negative) are all implemented in the GLSL shader.


## Step 2: the text texture is processed in GLSL

I'm using `regl` library helper and I need to inject the text canvas as a `uniform sampler2D text` parameter:

```js
uniforms: {
  text: regl.texture({ data: screenCanvas, flipY: true }),
```

After this, the main trick is to project the 2D Texture of the word onto the 3D raymarched object, and in my case, I simply project it along the Z-axis, globally. Indeed it would need to be applied "locally" if the Nano was actually moving or rotating but I didn't need that so we can simply stick to a global mapping.

So basically:

```glsl
vec2 coord = someOffset + someMultiplier * p.xy;
float m = step(texture2D(text, coord).x, 0.5);
```

makes `m` being a value of either 0.0 or 1.0 based on if the pixel is on or off.

Now, it also need to be pixelated, so we need to round the coordinate:

```glsl
vec2 coord = someOffset + someMultiplier * p.xy;
vec2 a = coord * vec2(128.,64.);
coord = floor(a) / vec2(128.,64.);
float m = step(texture2D(text, coord).x, 0.5);
```

Then we add a `edge` effect. This `edge` represents the distance to the edge of a pixel.

```glsl
vec2 coord = someOffset + someMultiplier * p.xy;
vec2 a = coord * vec2(128.,64.);
float edge = min(fract(a.x), fract(a.y));
coord = floor(a) / vec2(128.,64.);
float m = step(texture2D(text, coord).x, 0.5)
  * (1.0 - 0.5 * step(edge, 0.25)); // changes the pixel color
```

This will accentuate even more the pixel effect as we can see in this zoom:

<img src="/images/2022/gnsp/pixel.png" width="100%"/>

Ok, to precise exactly what `coord` is, here is the actual code:

```glsl
vec2 coord = fract(fract(vec2(-0.2, 0.5) + vec2(3.6) * p.xy / vec2(-2.25, 1.0)) + ${
  opts.scrollingScreen ? "vec2(0.5+floor(time*15.0)/15.0, 0.)" : "0.0"
});
```

You can note that:
- multiply by `vec2(-2.25, 1.0)` to stretch a bit the font.
- in case of `scrollingScreen` an offset by `time` is applied on x coordinate, and using some floor function so it does it by "increments" (pixel scroll)
- we apply a whole `fract` function (which is a `% 1.0`) to keep the coord in a 0.0 - 1.0 range and actually make it repeat.

Now, to create the negative effect, what we simply need to do is to either chose `m` or `1.0 - m` as a pixel value. 


This is implementing the simple idea to have half of the screen cut into two negative parts:

```glsl
${opts.halfnegativeScreen ? "m=mix(m,1.-m,step(coord.y, 0.5));" : ""}
```

And this implements the possibly blinking effect:

```glsl
${opts.blinkingScreen ? "m*=step(fract(2.*time),0.5);" : ""}
```

Now for the more complex animation, the effect varies at a given position on x,y, so we will give it to a function to determine if we need to swap the color. The animation we can see in the video above is a reference to one of my last year creation: [/shaderday/65](/shaderday/65).

```glsl
${
  !opts.screenAnimation
    ? ""
    : `
      float sz = ${(
        1 -
        opts.screenAnimation[3] * opts.screenAnimation[3]
      ).toFixed(2)};
      coord -= 0.5;
      coord *= vec2(2.,1.) * ${(
        1 -
        opts.screenAnimation[3] * opts.screenAnimation[3]
      ).toFixed(2)};
      coord += 0.5;
      ${
        opts.screenAnimation[1] < 0.2
          ? `coord.y${opts.screenAnimation[1] < 0.1 ? "+" : "-"}=time;`
          : ""
      }
      ${opts.screenAnimation[2] < 0.2 ? `coord.x-=time;` : ""}
      coord=fract(coord);
      m=mix(m,1.-m,step(shape(coord,2.*PI*time), 0.5));
    `
}
```


`screenAnimation` is a array of random values and with that, we can yield variation of the initial `shape` animation which is implemented relatively like in my [/shaderday/65](/shaderday/65):

```glsl
float shape (vec2 p, float t) {
  float smoothing = 0.15;
  p -= 0.5;
  vec2 q = p;
  pR(p, t + cos(${Math.round(5 * opts.screenAnimation[0] - 2)}. * t));
  vec2 dist = vec2(0.0);
  float crop = 99.0;
  float s = 99.0;;
  s = fOpUnionRound(q.y, s, smoothing);
dist = vec2(0.31, 0.0);
float radius = 0.11;
s = fOpUnionRound(s, length(p + dist) - radius, smoothing);
crop = fOpUnionRound(crop, length(p - dist) - radius, smoothing);
  s = fOpDifferenceRound(s, crop, smoothing);
  return smoothstep(0.0, 1.0 / min(resolution.x, resolution.y), s);
}
```

Finally, we map the "m" value to actual colors, and in our case it's basically black and white. Note the usage of `negativeScreen` flag:

```glsl
mix(
  vec3(0.01),
  vec3(1.0),
  ${opts.negativeScreen ? "1.-" : ""}m
)
```

> This GLSL code is templated in JavaScript as you may notice, it's a trick to make the GLSL compile even faster to avoid having runtime ifs.

**That's it folks! There are nothing more to say about the screen rendering of GNSP.**


<!--
// TODO explain multi platform in VIDEO article

```js
uniforms: { text: regl.texture(createImageTexture(screenCanvas))
```

where `createImageTexture` in context of web is:

```js
let createImageTexture = canvas => ({ data: canvas, flipY: true })
```

but for instance, in context of Node.js implementation is:

```js
let createImageTexture = (canvas) => {
  const ctx = canvas.getContext("2d");
  const width = canvas.width;
  const height = canvas.height;
  const imageData = ctx.getImageData(0, 0, width, height);
  return { data: imageData.data, width, height };
};
```
--->