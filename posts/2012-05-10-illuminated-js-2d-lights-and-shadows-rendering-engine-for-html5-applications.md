---
title: "Illuminated.js – 2D lights and shadows rendering engine for HTML5 applications"
description: Illuminated.js is designed to add some awesome effects to your existing applications. Adding a cool atmosphere for your applications and games can make the difference!
thumbnail: /images/2012/05/illuminatedjs.jpg
author: Gaetan
layout: post
permalink: /2012/05/illuminated-js-2d-lights-and-shadows-rendering-engine-for-html5-applications/
tags:
  - gamedev
  - canvas
  - javascript
  - library
---

[1]: http://bit.ly/LZ2dq1
[2]: http://gre.github.io/illuminated.js
[3]: http://github.com/gre/illuminated.js
[4]: /2012/05/illuminated-js-2d-lights-and-shadows-rendering-engine-for-html5-applications/#gettingstarted
[5]: /2012/05/illuminated-js-2d-lights-and-shadows-rendering-engine-for-html5-applications/#underthehood
[6]: http://en.wikipedia.org/wiki/Canvas_element
[7]: http://gre.github.io/illuminated.js/gettingstarted.html
[8]: http://en.wikipedia.org/wiki/Ray_tracing_(graphics)
[9]: /images/2012/05/step11.jpg
[10]: /images/2012/05/step21.jpg
[11]: /images/2012/05/step31.jpg
[12]: /images/2012/05/step4.jpg
[13]: http://blog.marmakoide.org/?p=1
[14]: /images/2012/05/sampling.jpg
[15]: http://en.wikipedia.org/wiki/Tangent_lines_to_circles

[![](/images/2012/05/illuminatedjs.jpg)][1]

[Click on the image to open it!][1]

## Wow! what’s this?

It’s a **2D scene** containing 2 **lights** and 13 different **objects** rendered in **real-time** by a **Javascript library** I made called **Illuminated.js**.

The library is designed to add some **awesome effects to your existing applications**. Adding **a cool atmosphere for your applications and games** can make the difference!

**[Try the editor][2]** and **[Get the source code][3]**.

In this article, we will introduce the basic usages of _Illuminated.js_ and APIs, and then explain how the engine works step-by-step.

- [API – Getting started][4]
- [Technical notes – how does it work?][5]

<!--more-->

## How can I use it?

The library uses [HTML5 Canvas][6] to draw lights and shadows – so you can simply drop it straight into your existing Canvas applications: you just need to add some code in your render function and maintaining a binding between your application logic and the _Illuminated.js_ objects.  
Not using canvas? No worries! In theory, if you have an existing application or game made in full DOM, you could use _Illuminated.js_ behind this, playing with z-index.

## <a id="gettingstarted"></a> Getting started

### Basic concepts

All the classes of the package live in `window.illuminated`.

A **Light** describes a light emit source.  
An **OpaqueObject** specifies an 2D object used by a Lighting.  
A **Lighting** defines the lighting of a light through a set of opaque objects, each object stops the light and casts shadows.  
A **DarkMask** defines a dark layer which hides dark area not lighted by a set of lights. It should be drown on the top-layer to hide objects which are far from the light. This effect produces a better atmosphere and is perfect for game where light are essential (where hiding invisible area is part of the difficulty).

### Example of a basic scene rendering

[  
Click here to open this example.  
![](/images/2012/05/gettingstarted.jpg)
][7]

## Lights and Objects

### Vec2

```javascript
new Vec2(x, y);
```

Vec2 represents a 2d position or a 2d vector. It is used everywhere in _Illuminated.js_.

Vec2 is inspired from Box2d’s Vec2 except that in _Illuminated.js_ a Vec2 vector is immutable. It means every methods create a new Vec2 instance and you can safely use a same Vec2 instance everywhere because the immutability guarantees the non-modification of properties.

### Lights

For now, we have only implemented one kind of light: a **Lamp** which is basically a radial gradient. A Lamp can also be “oriented”, it means lighting more far in a given direction.

#### Lamp

```javascript
new Lamp();

new Lamp({ position: new Vec2(12, 34) });
```

every parameters:

```javascript
new Lamp({
  position: new Vec2(12, 34),
  distance: 100,
  diffuse: 0.8,
  color: 'rgba(250,220,150,0.8)',
  radius: ,
  samples: 1,
  angle: ,
  roughness:
})
```

It defines a **Lamp** placed at a **position**, with a maximum emiting **distance**, a **diffuse** parameters to define the light penetration in objects.  
The **radius** defines the size of the light. Bigger the size is, Higher shadows are smoothed. The **samples** is an important parameters to define the quality of this smooth.  
The **angle** and **roughness** parameters are used for oriented lamp: angle defines the orientation while roughness defines the roughness of the effect.

### Light methods

You can easily create your own Light type by implementing its methods.

#### .mask(ctx)

Render a mask representing the visibility (used by DarkMask).

#### .render(ctx)

Render the light (without any shadows).

#### .bounds()

Return the Rectangle bound of the light representing where the light emission limit. `{ topleft: vec2, bottomright: vec2 }`

#### .forEachSample(fn)

Apply a function fn for each light sample position. By default it’s called once with the light position.

### Opaque Objects

In _Illuminated.js_, an object which cast shadows is called an opaque object. That’s why every types inherits OpaqueObject.

DiscObject and PolygonObject are the two available primitive objects.

#### DiscObject

A “DiscObject” is basically a 2D circlar object. You must define its center **position** and its **radius**:

```javascript
new DiscObject({ position: new Vec2(80, 50), radius: 20 });
```

#### PolygonObject

PolygonObject also has some derivated classes you can use: **RectangleObject**, **LineObject**.

You can instanciate these different objects like this:

```javascript
new PolygonObject([ new Vec2(, ), new Vec2(10, 10), ... ]) // an array of points
new RectangleObject(topleft, bottomright) // topleft and bottomright positions of the rectangle
new LineObject(a, b) // an object defined by the line from a to b.
```

### OpaqueObject methods

You can easily create your own object type by implementing OpaqueObject methods.

#### .bounds()

Return the Rectangle bound of the object. `{ topleft: vec2, bottomright: vec2 }`

#### .contains(point)

Return `true` if the object contains a **point**.

#### .path(ctx)

Build the path of the object shape in a 2d context **ctx**.

#### .cast(ctx, origin, bounds)

Fill every shadows with **ctx** projected by the **origin** point in the object and in a given **bounds**.

## Lighting and DarkMask

Previous defined classes was representing datas we will now use to perform lightings and masks.

### Lighting

A Lighting defines the lighting of one light through a set of opaque objects.

```javascript
new Lighting({ light: light, objects: [ object1, object2, ... ] })
```

#### .compute(width, height)

will compute shadows casting.

#### .cast(ctx)

will draw black shadows on the **ctx** canvas 2d context.  
You usually don’t have to use it if you use `render()`.

#### .render(ctx)

will draw the light with its shadows on **ctx** canvas 2d context.

### DarkMask

A DarkMask defines a dark layer which hides dark area not lighted by a set of lights.

```javascript
new DarkMask({ lights: [light1, light2, ...], color: 'rgba(0,0,0,0.9)' })
```

#### .compute(width, height)

will compute the dark mask.

#### .render(ctx)

will draw the computed dark mask on **ctx** canvas 2d context.

### about compute and render

Both Lighting and DarkMask objects have `compute()` and `render()` methods.

We think that **you** know the best when to recompute the lights because it’s closely link to the application you are making (we will not check at each time if something has changed, you know it).  
Call the `compute()` method when something has changed in your scene so we can recompute lights and shadows.

## <a id="underthehood"></a> How does it work under the hood?

_Illuminated.js_ divides its work into several layers.

### Real-time example

<iframe src="http://gre.github.io/illuminated.js/howdoesitwork.html" border="0" height="2700" width="450"></iframe>

### The art of composing layers

The layers are all stored in a Canvas which allows us to cache it. The light is drawn using a Canvas Radial Gradient in a cache canvas only once. This is interesting because canvas gradient are processor intensive  
At the end, layers are combine on the global canvas with `drawImage`.  
But the library lets you reuse these layers to combine them the way you want.

Canvas’ `globalCompositeOperation` is very useful to compose layers together.  
For instance, in the following example, the “Light shadow casting” layer is combined with the “Light rendering” layer to generate the “Light rendering with shadows” layer. The composition mode used is “destination-out” which remove the color of the destination image where the source image has color.

```javascript
light.render(ctx);
ctx.globalCompositeOperation = "destination-out";
this.cast(ctx);
```

Another very useful composite operation is `"lighter"` which adds color values. It is used to combine two lightings.

### How shadows are projected

Some rendering engine use [ray tracing][8] to render a scene, a concept very close to physics which trace from a light source a lot of rays with different paths which will collide with object and will be subject of absorption/diffraction/reflexion in accordance with the object properties…  
Ray casting is a very **realistic** rendering solution **but consuming** (you need a lot of rays to avoid noises in the result image).  
_Illuminated.js_ doesn’t use ray tracing because it aims to be efficient for a real-time usage. It uses some heuristics for casting shadows.

#### Let’s see how shadows are projected for a polygon object.

We have a scene with a light and a triangle.

![][9]

We select each edge of the polygon which is visible by the light (and in the light bounds).

![][10]

For every selected edge, we project it to generate a polygon area.

> **N.B.** In the current implementation, we generate an hexagon projection to ensure it goes outside of the light bounds because a quadrilateral didn’t garantee it, if a light is very close to it. The projecting vector used is enough big to work for most case, but it’s still an heuristic.

![][11]

We draw black color in this polygon area. Some improvments can be made by not drawing black in the shape / ajusting the opacity of the color.

![][12]

For casting blured shadows, we repeat this algorithm for each “samples” of the light. Samples are distribute around the light with a [“spiral algorithm”][13].

![][14]

```javascript
var GOLDEN_ANGLE = Math.PI * (3 - Math.sqrt(5));
Lamp.prototype.forEachSample = function (f) {
  for (var s = 0; s < this.samples; ++s) {
    var a = s * GOLDEN_ANGLE;
    var r = Math.sqrt(s / this.samples) * this.radius;
    var delta = new Vec2(Math.cos(a) * r, Math.sin(a) * r);
    f(this.position.add(delta));
  }
};
```

## To be continued…

The current version of _Illuminated.js_ needs more work, I’m aware of some bugs and some parts I need to improve:

- Implementing new kinds of lights like “Spot”, “Neon”, …
- The dark mask doesn’t follow the Lamp orientation.
- The shadow casting of Circle objects are not projected nicely, I need to compute [tangent lines to the circle][15].
- Shadows go sometimes wrong especially when having objects behind objects
- The shadow sampling implementation is a bit hacky and wrong (changing the samples parameter changes the shadow opacity…)

## Get involved

[Try the editor][2] and [Get the source code][3].

This article is translated to [Serbo-Croatian](http://science.webhostinggeeks.com/masina-za-renderovanje) language by Jovana Milutinovich from Webhostinggeeks.com.
