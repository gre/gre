---
title: 'Qep3.: QanimationFrame'
description: 'This third article on Q is a little parenthesis to the Qep articles series, featuring the requestAnimationFrame Javascript function and its general usage, and QanimationFrame, its Promisified version used as a "wait for DOM to be ready" API.'
thumbnail: /images/2013/07/qanimationframe.jpg
author: Gaetan
layout: post
tags:
 - AWOP
 - javascript
 - promise
 - Q
 - library
---

 [0]: /pages/a-world-of-promises/
 [1]: http://github.com/gre/qanimationframe
 [2]: https://dvcs.w3.org/hg/webperf/raw-file/tip/specs/RequestAnimationFrame/Overview.html
 [3]: http://creativejs.com/resources/requestanimationframe/
 [4]: http://www.paulirish.com/2011/requestanimationframe-for-smart-animating/

# A [World Of Promises][0], episode 3

<img src="/images/2013/07/qanimationframe.jpg" alt="" class="thumbnail-left" style="width: 200px" />

*This third article on [Q][1] is a little parenthesis to the Qep articles series,
featuring the `requestAnimationFrame` Javascript function and its general usage,
and [QanimationFrame][1], its Promisified version used as a "wait for DOM to be ready" API.*

<!--more-->

## `requestAnimationFrame`

`requestAnimationFrame` is a function which **delays a Javascript function execution to the next browser render frame**.
It takes one argument in parameters which is **the function to call on next repaint**.
*(N.B. there is not anymore a second DOM parameter like a few months ago, see the [spec][2])*

### ...for animation loop

`requestAnimationFrame` helps to easily make a **render loop**:

```javascript
(function loop(){
  requestAnimationFrame(loop);
  render();
}());
```
In that example, the `render` function can contains any Javascript code which updates
some graphics either using Canvas or DOM.

A good practice is to always **compute time-relative** animations and 
never assume the framerate to be constant.

```javascript
function badRenderFunction() {
 someObject.x += 0.1; // 10 pixels per 100 frame.
 // not so good with non-constant framerate
}
```

```javascript
var lastTime = Date.now();
function goodRenderFunction() {
 var now = Date.now();
 var delta = now-lastTime; // in milliseconds
 lastTime = now;
 someObject.x += 0.01 * delta; // 10 pixels per second
 // good because function of time
}
```

More information on `requestAnimationFrame` can be found [here][3] or [here][4].

### ...for waiting a DOM update

**We will now focus on another interesting benefit of that function:**

Instead of using `requestAnimationFrame` for a render loop,
**you can use it only once** in order to **wait for the next DOM update**.

There is a lot of use-cases where you need to wait for the next DOM update 
and `requestAnimationFrame` is perfect for that.

Most of the code you can see on the internet rely on using a `setTimeout` with an arbitrary time
given in second parameters *(sometimes 30, sometimes 0 !?)*.
This is, in my humble opinion, a wrong approach because you will never know if the repaint has 
really been performed.

## QanimationFrame

`QanimationFrame` is a function which takes a **DOM Element** in parameter and return a 
**Promise of that "ready" DOM element**.

**`QanimationFrame (elt: DOM Element) => Promise[DOM Element]`**

**N.B.:** Even if `requestAnimationFrame` doesn't have anymore a second *DOM element* parameter,
I found it quite cool that you can give it as argument and retrieve it back to manipulate it.
It also makes the function more composable because it behaves like an identity Promise function.
We will also see benefits when using with other DOM Promise libraries.

### Basic example

```javascript
var elt = document.createElement("div");
elt.innerHTML = "Hello world";
// wait for the DOM to be ready before using the height
QanimationFrame(elt).then(function (elt) {
  console.log("height="+elt.offsetHeight);
});
```

### Composability

```javascript
function createDivInBody (html) {
  var elt = document.createElement("div");
  elt.innerHTML = html;
  document.body.appendChild(elt);
  return elt;
}

var height = 
Q.fcall(createDivInBody, "Hello world!<br/>How are you today?")
 .then(QanimationFrame)
 .then(function (elt) {
   return elt.offsetHeight;
 });

height.then(function(height){
  console.log("height is "+height);
});
```

There is of-course a lot of more examples and use-cases of that library.

## Next episode

Next episode is a big one!

We will introduce you a Promisified animation library called **Zanimo.js** which
helps to chain different **CSS transitions with only Promises**.
It is very interoperable with any other Promise library,
meaning that you can easily chain Zanimo animations with other asynchronous actions.
