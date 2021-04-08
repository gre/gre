---
title: Blender as a 2D game level editor – Proof Of Concept
description: Here is how you can design and export your 2D game map with Blender (both the logic and the graphics).
thumbnail: /images/2012/04/map1.png
author: Gaetan
layout: post
permalink: /2012/04/blender-as-a-2d-game-map-editor-proof-of-concept/
tags:
  - gamedev
  - blender
  - javascript
---

#

A long time ago, video games were only two-dimensional. Of-course this was due to our poor hardware capabilities, but when computers became faster and faster 3D games appeared in mass.  
**Did it kill 2D games? Nope.** They continue to exist because it offer a different gameplay and are easier to make. Maybe also a bit because we are nostalgic of old-school games!

We can distinguish two kinds of 2D games:

<img src="/images/2012/04/bomberman93.jpg" alt="" class="thumbnail-left" />
[**Tile based games**][2] where the game world is simplified with a big grid – each grid position has some properties.  
A map editor is not always needed for tile based games, because the map can be straighforward to represent and maintain like in a *Bomberman* or in a *Pacman*. A simple editor is generally used to make graphism with sprites.

[2]: http://www.tonypa.pri.ee/tbw/tut00.html
[4]: http://higherorderfun.com/blog/2012/05/20/the-guide-to-implementing-2d-platformers/
[5]: http://www.masswerk.at/JavaPac/JS-PacMan2.html
[6]: http://impactjs.com/documentation/weltmeister
[7]: http://gre.github.io/blazing-race
[12]: http://gre.github.io/blazing-race/maps/converter/

<br style="clear:both" />

<img src="/images/2012/04/woarpc001.jpg" alt="" class="thumbnail-left" />
**Non-tile based games**, which can be called “polygon based games” are more complex.  
In such game, like a *Worms* or a *Sonic*, it’s totally crazy to write the map by hand (objects positions, polygons coordinates, …). The alternative, is not to use predefined maps, but on-the-fly generated maps which doesn’t fit every games.

<br style="clear:both" />
  
[Here are more detailed work on these different game designs][4].

**Making the game engine** is one thing, but **designing the game levels** can be one big work too and **we need tools to make it easier**.

<!--more-->

## Tile based game maps

In tile based games, maps are usually quite simple to represent.

For instance, here is how we can code the maze of [Pacman][5]:

```javascript
[
  "ahhhhhgxbhhdxehhhhhc",
  "vp....o......o....pv",
  "v.lhm...lhhm...lhm.v",
  "v.....n......n.....v",
  "v.n.n.v.ahhc.v.n.n.v",
  "d.v.o.v.vxxq.v.o.v.b",
  "x.v...v.vxxt.v...v.x",
  "c.bhm.o.bhhr.o.lhd.a",
  "v........x.........v",
  "em.lc.am.lm.lc.am.lg",
  "v...v.v......v.v...v",
  "v.k.o.o.lhhm.o.o.k.v",
  "vp................pv",
  "bhhhhhcxahhcxahhhhhd",
];
```

where every character is a tile and has a given meaning.

For more complex games, we can also represent the map with a set of objects, and each object has position and size properties (x, y, width, height) and other properties for the game logic.

For instance, see the _ImpactJS_ tile based games editor:

[![](/images/2012/04/weltmeister-tutorial-entities.png)][6]

## But what about polygons based game?

Well, some have tried to make dedicated 2D game map editor like shown in this video:

<iframe width="640" height="360" src="http://www.youtube.com/embed/kvvEmm2Vyoc?feature=player_embedded" frameborder="0" allowfullscreen></iframe>

but it sounds a bit unfinished and specific.

### Do it yourself, but don’t reinvent the wheel.

**But finally, isn’t it what a 3D editor is doing?**

Isn’t it the most generic tool we can find?

They have done a lot of awesome work in term of user interface, polygon modeling, textures (procedural / bitmap), …let’s profit of all this work to generate awesome texture map while exporting polygons.

Relying on such tools, you don’t have to learn a brand new map editor, you can relax on what you know if you have the chance to know Blender or Maya or anything.

### The Z magic

Let’s ignore the Z dimension, or rather, let’s **use the Z-dimension as a way to represent the semantics of the game map!**

This is the map I made for [Blazing Race][7], a HTML5 against-the-clock platform game where you control a fireball:

![](/images/2012/04/zs.png)

For my game needs, I used **different Z layers to represent different kind of materials and game objects**:

- z=1 : candles’ position – the objective of the game is to light them all
- z=0 : the game grounds – where collision occurs
- z=-1 : the water areas – where your flame dies
- z=-2 : special areas where you miss oxgyen – your flame dies in a few seconds

But I also used **objects ids** as an another way to distinguish objects:  
a “start” object to define the game start position and two “topleft” and “bottomright” objects to define the game bound.

### Maintain your map source in one file

Another powerful feature of this, is you can maintain your map polygons AND your map textures in a single way. Use your 3D editor as a polygon editor and use its render engine to generate textures:

![](/images/2012/04/map1.png)

Take benefits from what your 3D editor can do.

### Export polygons to the Javascript game

![](/images/2012/04/path4850.png)

I’ve made a transformer which take a COLLADA file in input (the most commonly supported standard format to describe a 3D scene, you can export it from any 3D editor like Blender, Maya, 3DS…) which extract and transform relevant informations from it and give you a json map for your game in output.

_It was quite simple to implement, thanks to the Three.js COLLADA importer!_

Here is the current (unfinished) interface for this:

[![](/images/2012/04/demo_screenshot.png)][12]

As a proof of usability of the output JSON map, the preview was only made in a few lines of Javascript code.

Extract:

```javascript
function draw(map) {
  var container = $("#viewport").empty();
  $("#legend").empty();
  a = 0;
  var w = 500;
  var h = Math.floor((w * map.height) / map.width);
  var CROSS_SIZE = 3;
  var canvas = $('<canvas width="' + w + '" height="' + h + '"></canvas>');
  var ctx = canvas[0].getContext("2d");
  for (var name in map) {
    var objs = map[name];
    if (objs[0] && objs[0].faces) {
      var color = randomColor(70, 0.8);
      ctx.fillStyle = color;
      for (var i = 0; i < objs.length; ++i) {
        var obj = objs[i];
        for (var f = 0; f < obj.faces.length; ++f) {
          var face = obj.faces[f];
          ctx.beginPath();
          for (var v = 0; v < face.length; ++v) {
            var vertice = obj.vertices[face[v]];
            var x = (ctx.canvas.width * vertice.x) / map.width;
            var y = ctx.canvas.height * (1 - vertice.y / map.height);
            if (v == 0) ctx.moveTo(x, y);
            else ctx.lineTo(x, y);
          }
          ctx.fill();
        }
      }
      addLegend(color, name, true);
    }
  }
  for (var name in map) {
    var objs = map[name];
    if (objs[0] && objs[0].x) {
      var color = randomColor(50);
      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      for (var i = 0; i < objs.length; ++i) {
        var p = objs[i];
        var x = (ctx.canvas.width * p.x) / map.width;
        var y = ctx.canvas.height * (1 - p.y / map.height);
        ctx.beginPath();
        ctx.moveTo(x - CROSS_SIZE, y);
        ctx.lineTo(x + CROSS_SIZE, y);
        ctx.moveTo(x, y - CROSS_SIZE);
        ctx.lineTo(x, y + CROSS_SIZE);
        ctx.stroke();
      }
      addLegend(color, name, false);
    }
  }
  container.append(canvas);
}
```

## What is next?

Blazing Race, is not finished yet, I need to improve a lot of things.

I’ll try to release a standalone version of this converter soon with tutorials and examples.
