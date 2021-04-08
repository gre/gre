---
title: "Play Painter – how i've improved the 30 minutes prototyped version"
description: One week ago, I’ve released a web experiment featuring a collaborative Paint-like application made with Play Framework 2 and relying on WebSocket and HTML5 Canvas. Here is how I've improved it.
thumbnail: /images/2012/03/twitt_playpainter.png
author: Gaetan
layout: post
permalink: /2012/03/play-painter-how-ive-improved-the-30-minutes-prototyped-version/
tags:
  - canvas
  - playframework
  - websocket
  - javascript
---

One week ago, I’ve released a [technical web experiment][1] featuring **a collaborative real-time Paint-like application I’ve called Play Painter**. It has been made with [Play Framework 2][2] and rely on WebSocket and HTML5 Canvas Javascript APIs.

[1]: /2012/03/30-minutes-to-make-a-multi-user-real-time-paint-with-play-2-framework-canvas-and-websocket/
[2]: http://playframework.org/
[4]: https://github.com/playframework/Play20/wiki/Iteratees
[5]: https://github.com/gre/playpainter/blob/master/scala/app/controllers/Application.scala
[6]: http://github.com/gre/playpainter
[7]: http://playpainter.greweb.fr/
[8]: https://github.com/gre/playpainter/issues/1
[9]: https://twitter.com/dbathily

Thanks to everyone having tested my Play Painter experiment, you helped me figure out bugs and bottlenecks and to benchmark the application running on my tiny server.  
The first version of Play Painter has been improved with some optimizations.

Explanation…

<blockquote class="twitter-tweet" lang="fr"><p>Thanks guys for testing playpainter! but you are breaking my server :D <a href="http://t.co/F62qwk1i" title="http://twitter.com/greweb/status/179194592481116160/photo/1">twitter.com/greweb/status/…</a></p>&mdash; Gaëtan Renaudeau (@greweb) <a href="https://twitter.com/greweb/status/179194592481116160">12 mars 2012</a></blockquote>

<!--more-->

## In brief

- **80 twitts**, **3500 unique visitors** in a few days.
- a peak of about **80 simultaneous painters**.
- about **200 WebSocket messages per second** when 3-4 users are drawing => bottleneck found.
- when it occurs, 100% CPU and about **1500 system interrupts per second** on my poor Atom 1.2 Ghz server.

## Some reasons

The initial version of Play Painter was a basic fast-prototyped version:

First, It was **spreading every mouse events** (down, up, move) to all clients **as fast as it comes**. It means that, depending on the computer and browser performance, a huge number of events could have been triggered and spread to all connected users.

We solved this by **Chunking draw events**.

Second, **a lot of informations was repeated in WebSocket messages.** No datas were stored on the server-side so to be sure a new user see the right draws, player name, brush color and size was sent in every message. Multiply this by the number of mouse events and you get a lot of useless information!

We are now **Storing painters information**.

## Chunking draw events

When an user starts drawing, mouse events give the brush positions (x, y). But instead of sending a websocket message for each of these new positions, they are stored, and every **X** milliseconds, are sent in a websocket message. Such message contains all points of the draw from the last sent draw message.

The **X** value has currently been fixed to **50 milliseconds** because it’s enough for the human eye: It means about 20 messages per second for one painter. In movies we usually have a 24 frame rate.

The same principle has been applied on the painter brush positions.

### Example

**Before**  
13 WebSocket messages:

```javascript
{"type":"lineTo","x":181,"y":259,"pid":19}
{"type":"lineTo","x":183,"y":259,"pid":19}
{"type":"lineTo","x":184,"y":257,"pid":19}
{"type":"lineTo","x":187,"y":257,"pid":19}
{"type":"lineTo","x":188,"y":257,"pid":19}
{"type":"lineTo","x":191,"y":256,"pid":19}
{"type":"lineTo","x":192,"y":255,"pid":19}
{"type":"lineTo","x":192,"y":255,"pid":19}
{"type":"lineTo","x":192,"y":254,"pid":19}
{"type":"lineTo","x":193,"y":254,"pid":19}
{"type":"lineTo","x":195,"y":254,"pid":19}
{"type":"lineTo","x":196,"y":253,"pid":19}
{"type":"lineTo","x":196,"y":253,"pid":19}
```

**After**  
2 WebSocket messages: (50 ms apart)

```javascript
{"type":"trace","points":[{"x":181,"y":259},{"x":183,"y":259},{"x":184,"y":257},{"x":187,"y":257},{"x":188,"y":257},{"x":191,"y":256},{"x":192,"y":255}],"pid":19}
{"type":"trace","points":[{"x":192,"y":255},{"x":192,"y":254},{"x":193,"y":254},{"x":195,"y":254},{"x":195,"y":253},{"x":196,"y":253},{"x":196,"y":253}],"pid":19}
```

### To a variable frame rate?

I am also thinking about a variable rate depending of the number of active painters. In fact, the more we have painters, the more we will have messages, and the more the server will have system interrupts, we could then decrease the frame rate per second to reduce this load.

The problem of this extreme approach is the degradation of the feeling of real-time.

For now, I’m keeping the constant frame rate version, we will see how far it goes.

## Storing painters information

As I said, **a lot of informations was repeated in WebSocket messages.** In every draw events, painter name, brush size and brush color was sent from the client to the server, and the spread into all connected clients.

This was ok for prototyping but we have now optimize this by storing these painter generic informations in the server and sending them when a new WebSocket connection is opened.

### Example

This is what a client can receive when a websocket is connected:

```javascript
{"type":"youAre","pid":24}
{"name":"john","color":"red","size":5,"type":"painter","pid":21}
{"name":"gre","color":"red","size":5,"type":"painter","pid":24}
{"name":"peter","color":"red","size":5,"type":"painter","pid":4}
{"name":"paul","color":"red","size":5,"type":"painter","pid":6}
{"name":"jack","color":"red","size":5,"type":"painter","pid":2}
```

and then…

```
{"type":"trace","points":[{"x":181,"y":259},{"x":183,"y":259},{"x":184,"y":257}],"pid":19}
...
```

By knowing all painter properties, when someone will draw something, he will not have to repeat which color and size its brush has.

### Server side

It was quite interesting to implement the server part with [Play2′s Iteratees][4], a new way of handling I/O – not so new in fact because it is directly related to Haskell Iteratee concepts.

To implement a WebSocket connection, you will provide an **Iteratee** for consuming the **input** and an **Enumerator** for producing the **output**.

Enumerator are chainable, this is how I firstly send the painter id and painters informations:

```scala
// out: handle messages to send to the painter
val out =
  // Inform the painter who he is (which pid, he can them identify himself)
  Enumerator(JsObject(Seq("type" -> JsString("youAre"), "pid" -> JsNumber(pid))).as[JsValue]) >>>
  // Inform the list of other painters
  Enumerator(painters.map { case (id, painter) =>
    (painter.toJson JsObject(Seq("type" -> JsString("painter"), "pid" -> JsNumber(id)))).as[JsValue]
  } toList : _*) >>>
  // Stream the hub
  hub.getPatchCord()
```

The **>>>** operator is a shortcut to the **andThen** method which is the way to chain enumerators.

For more details, [see the scala code of the controller][5].

## Other features

The application has been improved in many other ways.

- **A “buffering” Canvas** in the foreground has been add **for the user draws**. It brings client-side reactivity and helps to avoid unpleasant lag feeling when drawing. When the user draw events are coming from the server and no other user events has been sent since, it’s synchronized and we can clean this buffer.
- **Painter positions are show with their names**.
- It should now work properly on **smartphones and tablets**. Try on iPad and iPhone, and maybe on recent version of Android (WebSocket support required).
- **Keyboard shortcut**: using arrows to change brush size and color.
- The **[source code][6] has been polished and commented** especially the server side part (it’s probably the hardest part if you don’t know Play framework).
- Error message displayed when a technology is not supported and when the WebSocket connection goes down (with a reconnecting try loop).

## The demo is still online!

**[playpainter.greweb.fr][7]**

## Future

With these two optimizations, I’ve reduce the global **number of socket messages** and also the **size of each message**.

The first benchmark sounds good, 3 painters was simultaneously crazily painting while the server application was only using less than 10% of CPU.

Now, the most challenging part would be to scale the application to a huge number of connections, but having maybe solved this bottleneck, it’s maybe now more a matter of system architecture than the application itself.

This experiment gave me a lot of interest in **WebSocket** and also in **the powerful way WebSockets are handled in Play framework**.

If anyone want to start a Java version of the application, please go on! [(this was requested on Github)][8]

Thanks to [@dbathily][9], we know have both Scala and Java version!

### Next experiment

I am thinking about making a multiplayer game on the web.  
It would be something like a shooter survival game (like Counter Strike Zombie Mod) a multi-plateform 2D side view game (like Mario) !

You will know more about this soon!
