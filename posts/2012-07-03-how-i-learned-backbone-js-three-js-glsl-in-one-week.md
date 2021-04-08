---
title: "How I learned Backbone.js, Three.js, GLSL in one week"
author: Gaetan
description: I learned to use Backbone.js and Three.js (a famous library on top of WebGL) to make a FPS in only one week.
thumbnail: /images/2012/07/dta1.png
layout: post
permalink: /2012/07/how-i-learned-backbone-js-three-js-glsl-in-one-week/
tags:
  - gamedev
  - GLSL
  - WebGL
---

[1]: http://youcanmakevideogames.com/
[2]: http://7dfps.org/
[3]: http://www.ludumdare.com/
[4]: http://gre.github.io/dta
[8]: http://caniuse.com/#search=webgl
[12]: http://www.egonelbre.com/js/jsfx/
[16]: http://www.opengl.org/documentation/glsl/
[17]: http://glsl.heroku.com/
[19]: https://github.com/gre/dta/blob/master/app/models/models.scala
[20]: http://playframework.org/

![](/images/2012/07/dta1.png)

Last week was the [7dfps challenge][2], an open challenge where participants had to make a FPS in only one week.
Such contest are very very interesting for those who want to experiment with things. Challenging yourself is IMO the best way to learn new things. You may also know the famous [“Ludum Dare” contest][3].

I learned to use Backbone.js and Three.js (a famous library on top of WebGL) in only one week, so you have no excuse to not be able to do the same!

[-> You can make games!][1]

> “If Lawnmower Man f\*\*\*\*d Tron on the bonnet of a tank”  
> **_YouBigNugget_**

I’ve only used web technologies, no need of any plugin, just a recent browser like Chrome / Firefox.

This is the result:

<iframe width="640" height="360" src="http://www.youtube.com/embed/g9CldBI9C6E?feature=player_embedded" frameborder="0" allowfullscreen></iframe>

and you can play it here:

**[Play the game][4]**

<!--more-->

## Overview of a one week game development

### Backbone.js, for the model, events and class inherence

I’m a fan of the “do it yourself” idea, not using any library or only using small 140byt.es codes. But the frequent issue with that is (1) always doing the same thing again and again, (2) taking a lot of time on the architecture, and not finishing anything. When you have a due date, relying on libraries and frameworks could save you a lot of time.  
I’ve choose Backbone.js for its Model architecture with class inherence, a get/set system, and also its event system bound to model instances, the destroy function with the “destroy” event to bind on,… It was also a new library to learn for me.

#### Models

Here is the class diagram of the game:

![not available](/images/2012/07/f5dea341.jpg)

### Learning Three.js, a 3D library on top of WebGL.

WebGL is [more and more supported by browsers][8].  
WebGL means OpenGL in the web. It allows to make efficient and hardware accelerated 3D computation.

Here is the same, not using any library but using pure WebGL wasn’t possible in one week! This is why I’ve chosen to use Three.js, probably today the most popular WebGL library.  
I was impressed how Three.js is finally not so hard to use, knowing some basic 3D concepts from my old Blender days.

You create a `Scene`, add a `Camera`, add Meshes to the scene, A `Mesh` has a `Material` and a `Geometry`, … everything very straighforward.

One challenging part was when trying to compute global world position relative to a given object position, for instance to compute the Bullet initial position and orientation from the Tank position and orientation.

Fortunately I’ve got some help ![:)][4] and got the answer: multiply your vector with the worldMatrix of the mesh.

<blockquote class="twitter-tweet" data-in-reply-to="212609230891524096" width="550" lang="fr"><p>@<a href="https://twitter.com/greweb">greweb</a> Should be calculatable quite easily. Multiply the position vector by the object’s matrixWorld property.</p>
<p>&mdash; Paul Lewis (@aerotwist) <a href="https://twitter.com/aerotwist/status/212617930024816641" data-datetime="2012-06-12T18:50:39+00:00">Juin 12, 2012</a></p></blockquote>

<blockquote class="twitter-tweet" data-in-reply-to="212609230891524096" width="550" lang="fr"><p>@<a href="https://twitter.com/greweb">greweb</a> var position = new THREE.Vector3().getPositionFromMatrix( object.matrixWorld );</p>
<p>&mdash; Mr.doob (@mrdoob) <a href="https://twitter.com/mrdoob/status/212648943673290752" data-datetime="2012-06-12T20:53:53+00:00">Juin 12, 2012</a></p></blockquote>

### Playing with AI

Like TankKeyboardControls, I’ve created a new “TankControls” for computer tanks: **TankRandomControls** was the first dumb one, it just does random things:

```javascript
function TankRandomControls () {
var self = this;
self.moveForward = false;
self.moveBackward = false;
self.moveLeft = false;
self.moveRight = false;
self.fire = false;
self.fireMissile = false;
var i = ;
setInterval (function () { // take random decisions every 0.5 second
  i;
self.moveForward = i%3== && Math.random() > 0.2;
self.moveBackward = !self.moveForward && Math.random() > 0.5;
self.moveLeft = Math.random() < 0.1;
self.moveRight = Math.random() < 0.1;
self.fire = Math.random() > 0.2;
self.fireMissile = Math.random() < 0.2;
}, 500);
}
```

**TankRemoteControls** was the second one using some simple AI rules:

- Try to avoid walls and objects
- Either do random things or target a tank (more likely)
- Target the closest tank, update target every 5 seconds
- Turn the tank to the target tank, shoot bullets and missiles
- Move forward if the target is far away / Move backward if the target is too close

See the source code here: .

### Sounds

I’ve used [JSFX][12], an experimental library, were you can generate sounds based on a few parameters.  
It’s a 8-bit sound generator perfect for generate old-school sounds.

So you can create each sound in Javascript like this:

```javascript
SOUNDS = {
  bullet: jsfxlib.createWave([
    "noise",
    7.0,
    0.18,
    0.0,
    0.082,
    0.0,
    0.222,
    20.0,
    800,
    2400.0,
    -0.428,
    0.0,
    0.0,
    0.01,
    0.0003,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.992,
    0.0,
    0.0,
    0.22,
    0.0,
  ]),
  missile: jsfxlib.createWave([
    "noise",
    0.0,
    0.12,
    0.0,
    0.546,
    0.0,
    0.856,
    64.0,
    250,
    1063.0,
    0.28,
    0.086,
    0.024,
    5.4329,
    0.3565,
    0.466,
    -0.638,
    0.058,
    0.008,
    0.0,
    0.0,
    -0.114,
    0.216,
    0.98,
    -0.984,
    1.0,
    0.307,
    0.988,
  ]),
  explosion: jsfxlib.createWave([
    "noise",
    1.0,
    0.4,
    0.02,
    0.682,
    1.746,
    1.97,
    100.0,
    378.0,
    2242.0,
    -0.55,
    -0.372,
    0.024,
    0.4899,
    -0.1622,
    0.262,
    0.34,
    0.724,
    0.0205,
    -0.102,
    0.0416,
    -0.098,
    0.1,
    0.805,
    0.094,
    0.428,
    0.0,
    -0.262,
  ]),
  collideWall: jsfxlib.createWave([
    "noise",
    0.0,
    0.4,
    0.0,
    0.056,
    0.0,
    0.296,
    20.0,
    560.0,
    2400.0,
    -0.482,
    0.0,
    0.0,
    0.01,
    0.0003,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    1.0,
    0.0,
    0.0,
    0.0,
    0.0,
  ]),
};
// ...
// SOUNDS.bullet.play()
```

The jsfxlib.createWave function returns a HTML5 Audio element and you can play with its API (.play(), .pause(), …).  
But doing this way, you will not be able to play a sound at the same time so I’ve made a buffer system which duplicate N times the sound in different audio elements.  
I have also try to generate different sounds to randomize it a little.

See the full source code here: .

### Post Processing Effects with GLSL shaders

I was very impressed by the power of GLSL for its possibilities and efficiency.  
GLSL are definitely the indispensable thing you need to add a better atmosphere in your games.

#### Before

![](/images/2012/06/before.png)

#### After

![](/images/2012/06/after_.png)

and when getting shot:

![](/images/2012/06/after.png)

These effects are done by combining these two GLSL effects.  
Here is the GLSL code of these effects.

#### The radio noise effect

<script src="https://gist.github.com/2950478.js?file=radionoise.frag"></script>

#### The shot interference

<script src="https://gist.github.com/2950478.js?file=perturbation.frag"></script>

### Wait! what is this crazy “GLSL” language?

GLSL is an OpenGL language close to the C syntax design to have a direct control of the graphic pipeline. You can directly add passes to the rendering process with GLSL shaders.  
GLSL gives you a lot of very useful types (like vectors, matrix, …) and functions (like smoothstep, …). [Read the spec][16] to know more about it.  
I also recommend you to experiment the [GLSL Sandbox][17]. You can see awesome demos and their codes shared by people made with GLSL. It also have an online IDE to easily code and instantly test your shaders.

### WebGL and Three.js integration

You can use GLSL in different ways with Three.js. You can add a GLSL shader to an object material, and you can also add GLSL shaders on top of the Canvas (post-processing). We will only see how to add GLSL shaders as a post processing render on the entire Canvas.

Fortunately Three.js have some utils classes to make the GLSL shaders integration easier.

I have been inspired from the awesome work done here: .

There is a lot of code out there, so I’ve try to extract the minimum required for mixing the 2 GLSL shaders for a post-processing on the entire canvas of a Three.js scene:

<iframe style="width: 100%; height: 400px;" src="http://jsfiddle.net/jggvJ/7/embedded/" frameborder="0" width="320" height="240"></iframe>

As you can see, you can easily inject your own Javascript variables in a GLSL shader to make a bridge between the JS and the GLSL code and so having quite generic and configurable shaders.  
Quite cool!

### Adding game UI

Web, with HTML and CSS, allows to have different containers, layers, positioning systems,…  
Perfect! For the game UI, we used different elements:

![](/images/2012/06/ui.png)

Radar and Damage are implemented with an independent Canvas while Level is a simple text div.

For instance, this is Damage (I called LifeIndicator):

```javascript
(function(){

var LifeIndicator = function (nodeId) {
this.canvas = document.getElementById(nodeId);
this.ctx = this.canvas.getContext("2d");
this.life = 1;
this.render();
}

LifeIndicator.prototype.setLife = function (life) {
this.life = life;
this.render();
}

LifeIndicator.prototype.render = function () {
var c = this.ctx;
var w = c.canvas.width, h = c.canvas.height;
c.clearRect(, , w, h);
// TODO
var g = Math.floor(255*this.life);
var r = 255-g;
c.fillStyle = "rgb(" r "," g ",0)";
var wt = 6;
var ht = 12;
c.fillRect((w-wt)/2, , wt, ht);
c.fillRect(, ht, w, h);
}

window.LifeIndicator = LifeIndicator;

}());
```

## Play Framework integration

I was planning to make a multiplayer game but I couldn’t find the time for this.  
I use Play Framework 2 and its power and concepts for handling streams (even if I don’t use it yet).  
The only part I can show you for now is the game map generation. For making a multiplayer game, I needed to have the game state on server side to be able to submit game infos (map, players, …) to new players.

This is a few scala code to generate a nice distribution of random objects in the map:

```scala
object Game {
  def createRandom: Game = {
    val random = new Random()
    val half = 5000
    val size = 2*half
    val split = 4
    val objects =
    Range(, split).map { xi =>
      Range(, split).map { yi =>
        val s = size.toDouble / split.toDouble
        val x = -half.toDouble math.round(s*(xi random.nextDouble));
        val y = -half.toDouble math.round(s*(yi random.nextDouble));
        var sizeRandom = random.nextDouble
        val w = 200.*math.ceil((1-sizeRandom)*8.);
        val h = 200.*math.ceil(sizeRandom*8.);
        RectObj(Vec3(x,y,), w, h)
      }.toList
    }.toList.flatten
    Game( (Vec3(-half, -half, ), Vec3(half, half, )), List(), objects, List() )
  }
}

case class Game (bounds: Tuple2[Vec3, Vec3], chars: List[Char], objects: List[GameObj], dynamics: List[GameDyn])
...
```

[See the full source][19] (some code may not be used).

## Still an unfinished game

There is more things to do now:

- More visual effects
- More sound effects
- A better collision system (using a physic engine?)
- Multiplayer
- A better gameplay with some goals, different kind of games, …

### Initial multiplayer objective rescheduled

My initial game release was focus on the game core, graphisms and gameplay.

I was unfortunately not ready enough to make the multi-player real-time part for the first week sprint but I have some though on the subject.  
I was asking myself what are the best way to make the game synchronisation between clients while trying to keep as much client-side code as possible. I wanted a scalable decentralized game. I had some though on how to solve this issue. For instance, when a client shoots, he sends a “shoot” event with the timestamp, server just streams events (with a tick frequency) sent by clients and clients replay the game exactly the same way everywhere (using all these time-based events) with some interpolation with the current time.  
I need to think more about this now, and try to use [PlayFramework][20].

## Source code

[http://github.com/gre/dta](http://github.com/gre/dta)

**[EDIT] You may be mainly interested by the [main.js][21]. It shows how powerful the even-driven programming is, for plugging components together.  
**

[21]: https://github.com/gre/dta/blob/master/app/assets/javascripts/main.js

## Special thanks

[@mrspeaker][22] for helping me with Three.js & also collision system,  
[@mrdoob][23] & [@aerotwist][24] for replying to my newbies questions,  
guys on IRC (#three.js),  
and of-course 7dfps guys.

[22]: http://twitter.com/mrspeaker
[23]: http://twitter.com/mrdoob
[24]: http://twitter.com/aerotwist
