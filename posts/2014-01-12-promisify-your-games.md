---
title: 'Promisify your games'
description: 'a game showcase using Q Promise as first-class citizen and driven with CSS3 Animations via Zanimo.'
thumbnail: /images/2014/01/ld28.png
author: Gaetan
layout: post
tags:
 - gamedev
 - promise
 - Q
 - ludumdare
---

 [promise]: /2013/07/q-a-promise-library/
 [ld]: http://www.ludumdare.com/compo/
 [game]: http://greweb.me/ld28/
 [qimage]: https://npmjs.org/package/qimage
 [qajax]: https://npmjs.org/package/qajax
 [submission]: http://www.ludumdare.com/compo/ludum-dare-28/?action=preview&uid=18803
 [zanimo]: https://github.com/peutetre/Zanimo
 [npmjs]: https://npmjs.org/
 [github]: https://github.com/gre/ld28
 [app.js]: https://github.com/gre/ld28/tree/master/src/app.js

One month ago was the [LudumDare][ld] #28 gamejam theming *"You Only Get One"*.

<a href="http://greweb.me/ld28/">
  <img src="/images/2014/01/ld28.png" alt="" class="thumbnail-left" />
</a>

I [submitted][submission] a [mini-game][game] which ranked 105th out of 2064 entries and also 26th in the "theme" category.

This is of-course a web game implemented in JavaScript and using HTML and CSS.

But actually, my main goal was not really making a game done 
but more about technically **making a state-of-the-art Promise-based game**.

I think [Promises][promise] contains very interesting advantages in a game development design:
*Resource loading managment*, *game scenes chaining*, *animations*... are some use-cases.


* Checkout the [source code][github] on Github - [`src/app.js`][app.js] is the entry point
* LudumDare entry is [here][submission].
* [Play the Game][game].


<!--more-->

## FP in game development

Using some Functional Programming paradigm in game development is interesting,
and here I'm just talking at least about the basic stuff:
**Avoid globals, Minimize state variables**.

I've written a few games where restarting the game without using `location.reload()` was a challenge
because the game variable states was so spread everywhere!

By doing more FP, you can have this restart feature by design without need to "reset" all variables
because your start function just takes everything it needs in parameter and restarting is just about re-calling that function.

## Promises as first-class citizen

### Game scenes chaining

Like maybe 99% of games, my game has an **intro** (menu), a **main** scene and an **outro** scene (gameover / finish).

When you develop a game with just one big `render()` loop,
it easily becomes a pain when you want to add more steps to the scene,
it doesn't scale and fastly become spaghetti code:
you tend to have to figure out in which state you are (or which part of the animation timeline) from the game state.

Hopefully, scene management is very easy to do with Promises:

```javascript
function start () {
  return Q()
    .then(intro)
    .then(_.partial(runMiniGames, 20))
    .then(outro);
}

Q.all([/*..something to load..*/])
 .then(start/*, ..*/) // start game when ready
 .done(); // just help Q to trigger errors if some.
```

How beautiful to read! Call `intro` then run 20 mini-games then perform `outro`.

#### No game state shared, pure functions

**intro** is the menu screen where you can choose the game difficulty.
**outro** is the game end screen where the final score is displayed.
There is however **no global variables shared**, those are just passed from one function to another.

Let's look deeper in how it works:

* The `intro()` function just returns a Promise resolved when the user made a "difficulty" choice. That Promise actually contains the difficulty (0, 1 or 2).
* The `runMiniGames` function takes 2 parameters: the number of games and the difficulty. `_.partial(runMiniGames, 20)` is just an helper for making a 20 mini-games function which takes the difficulty in parameter. This difficulty is given by the previous Promise. The `runMiniGames` function returns a Promise of Score (integer).
* That score is then fed into the `outro(score)` function which displays this score to the user.

> **TL;DR.** This is just about plumbing 3 functions together!

Checkout also the [`runMiniGames` implementation](https://github.com/gre/ld28/blob/master/src/app.js#L24-L37).

#### Speed up the development

And you know what? It make development easier and faster because you can easily skip some part in any Promises chain:

```javascript
function start () {
  return Q(0)
    //.then(intro) // Directly jump to the games
    .then(_.partial(runMiniGames, 20))
    .then(outro);
}
```

I used that a lot and not only for this part, but for all part during the game development.


### Loading resources

Promises also help you to wait resources before starting the game.
You don't have to make yet another loading library, Promise already are ready for that, 
and you can also have proper error managment or even "progress" loading display (Q has Progress event in a Promise).

Each loading resource is a Promise and you can combine them all using `Q.all`.

Here is an example using [Qajax][qajax] and [Qimage][qimage].

```javascript
Q.all([
  Qajax("music.wav").then(mapToAudio),
  Qajax.getJSON("map.json"),
  Qimage("images/logo.png"),
  Qimage("images/textures.png")
]).spread(function (music, map, logo, textures) {
  
  // Start the game !

}, function (error) {

  // display proper error

}, function (progressEvent) {

  // maybe you want to display a loading progress bar 
  // with that third progress callback.

});
```

Here is a similar example:

```javascript
var musicPromise = Qajax("music.wav").then(mapToAudio);
var mapPromise = Qajax.getJSON("map.json");
var texturesPromise = Qimage("images/textures.png");
Q.all([ mapPromise, texturesPromise ]).spread(startGame, errorLoading);
// because maybe you don't want to wait the music for starting the game:
musicPromise.then(function (a) { a.play(); });

function startGame (map, textures) {
  // ...
}
function errorLoading (e) {
  // ...
}
```

### Mini games workflow

My games is divided into a set of mini-games which are all independent but share a common interface.
This interface was quite a WIP at the end of the weekend development but it does the job.

Here is the template I used for my game: [src/games/\_template.js](https://github.com/gre/ld28/blob/master/src/games/_template.js).

A Game instance has different methods, and especially `enter` and `leave` method which are call on game enter and on game leave. It also has a `.end` Promise which is resolved when the Game end.

* A mini-game when solved gives a score depending on how well the user succeed it (through the `end` Promise).
* A mini-game have a timeout and if the player doesn't terminate it, it passes to the next game without scores.

Those `enter()` and `leave()` methods return Promise in order to be plugged in the game workflow (we can wait them to finish before moving to next state).

For instance, we don't start the game timeout before it actually starts (just wait the `enter()` Promise to be resolved) and also we don't switch to the next game before the `leave()` Promise is done).

Checkout also the [`nextMiniGame` implementation](https://github.com/gre/ld28/blob/master/src/app.js#L51-L76).
The result of that function is the score of the mini-game and that we sum up all scores from the previous score.

#### Composability

The `enter()` and `leave()` methods can be composed of animations which can themselves be composed of animations.

**We can easily subdivided work into different level of Promises chain.**
Here is a little schema to summary that composability:

![](/images/2014/01/ld28_composition_schema.svg)

### Promise Animations

In my game, all the animations are controlled with Promises more exactly using CSS3 Transitions 
via **[Zanimo][zanimo] Promise library** because it fits my game (DOM-based game).
The fact that a Promise can be waited and chained **gives a powerful controls over CSS Transitions for making animations**.

You can easily trigger animations **one after another** for moving an element in multiple places.
You can also perform **multiple animations at the same times** (on 2 different elements) and **wait for both to finish**
before triggering a third animation.

See for instance how `enter()` and `leave()` animations are done in mini-games.

In the animation ending the "memo" game I used concurrent animations:
all memo cards are randomly moved out.

```javascript
/* // FYI
Card.prototype.transform = function (x, y, scale, duration) {
  return Zanimo.transition(this.el, "transform",
    "translate("+x+"px, "+y+"px) scale("+scale+")", duration||0);
};
*/

function animateOut (dispersion) {
  return Q.all(_.map(cards, function (card) {
    if (card.destroyed) return Q(); // no animation because card is destroyed
    return Q()
      .then(function(){
        return card.transform(card.x, card.y, card.number === 1 ? 1 : 0.8, 100);
      })
      .delay(Math.floor((card.number===1 ? 500 : 0)+300*Math.random()))
      .then(function () {
        var x = Math.round((Math.random()<0.5 ? -card.w/dimensions.width-dispersion*Math.random() : 1+dispersion*Math.random())*dimensions.width);
        var y = Math.round((Math.random()<0.5 ? -card.h/dimensions.height-dispersion*Math.random() : 1+dispersion*Math.random())*dimensions.height);
        return card.transform(x, y, 0, 500);
      });
  }));
}

// Usage in Memo.leave() :
return Q.delay(50)
  .then(function(){ return animateOut(0.5); })
  .delay(100);
```


In the calculation game I used a chain of animations subdivided in functions:

```javascript
return Q.delay(50)
  .then(fadeOutInvalids)
  .then(displaySolution)
  .then(displayEquality)
  .delay(500)
  .then(fadeOut)
  .then(hideEquality)
  .delay(200);
```

[Full code here](https://github.com/gre/ld28/blob/master/src/games/calculation.js#L411-L500).

### "Wait for next click"

While my game are just based on click user interaction,
I've made a [`waitNextClick`](https://github.com/gre/ld28/blob/master/src/waitNextClick.js) function
which returns a Promise of click for the given element.

```javascript
var Q = require("q");

module.exports = function waitNextClick (btn) {
  var d = Q.defer();
  btn.addEventListener("click", function listener (e) {
    btn.removeEventListener("click", listener);
    d.resolve(e.target);
  }, false);
  return d.promise;
};
```

This was quite an interesting solution which is just like a jQuery "once" event but in Promise paradigm.

I was able to combine that function with `Q.race` which wait for one of the given Promise to be redeemed.

> `Q.race(_.map(btns, waitNextClick))`

For instance in the cats game, I just wait the first "This one" button to be clicked:

```javascript
var houseChoice = Q.race(_.map(this.houses, function (catHouse) {
    return waitNextClick(catHouse.btn)
    .then(function () {
      return catHouse;
    });
// houseChoice is a Promise of House choosen by the player.
```

### Using the "progress" event

I also used a bit the "progress" event of a Q Promise, which is a way to notify that a Promise is being resolved.

* I used that "progress" event on the `game.end` Promise for notifying that the player is winning some scores in a mini-game while playing.
* I also used it to make a timeout ticking the remaining time before the timeout is reached and that Promise resolved.

See both usages [here](https://github.com/gre/ld28/blob/master/src/app.js#L62-L70):

```javascript
return Q.race([
  gameEnd
    .progress(function (score) {
      stats.setScore(totalScore+score);
    }),
  timeoutWithTicks(gameEnd, timeout)
    .progress(stats.setTimeProgress)
    .then(_.bind(game.submit, game))
]);
```

## Code organization using NPM + Browserify

NPM & Browserify has also been used because I find this stack very productive,
especially when writing a game from scratch.

Browserify has been trendy the last past year, but there is here an interesting way of organizing your code
and especially reusing it.
You can find a lot of [available modules using NPM][npmjs], 
Browserify will just make you able to require them using `require("modulename")`.

