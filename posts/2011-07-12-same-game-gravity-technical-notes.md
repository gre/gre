---
title: 'Same Game Gravity: 6 platforms, 1 codebase'
thumbnail: /images/2011/07/gravity_exemple.png
description: The Same Game is available for 6 different platforms. And I can pump out new builds for them all in around 15 minutes. Here’s how…
author: Gaetan
layout: post
permalink: /2011/07/same-game-gravity-technical-notes/
tags:
  - mobile
  - gamedev
  - canvas
---

 [1]: /2011/07/same-game-gravity-for-ipad-iphone-android-facebook-chrome-and-web/
 [2]: http://gre.github.io/same-game-gravity
 [4]: /2011/06/automating-web-app-development-for-multiple-platforms/
 [5]: https://github.com/gre/same-game-gravity
 [6]: https://github.com/gre/same-game-gravity/blob/master/game.html
 [7]: https://github.com/gre/same-game-gravity/blob/master/game.css
 [8]: https://github.com/gre/same-game-gravity/blob/master/game.js
 [9]: https://github.com/gre/same-game-gravity/blob/master/game.js#L324
 [10]: https://github.com/gre/same-game-gravity/blob/master/game.js#L687
 [11]: https://github.com/gre/same-game-gravity/blob/master/game.js#L850
 [12]: https://github.com/gre/same-game-gravity/blob/master/game.desktop.js
 [13]: https://github.com/gre/same-game-gravity/blob/master/game.desktop.js#L137
 [14]: http://docs.phonegap.com/phonegap_accelerometer_accelerometer.md.html
 [15]: http://dev.w3.org/geo/api/spec-source-orientation.html
 [16]: http://twitter.com/42loops
 [17]: https://github.com/peutetre/test-mobile-safari/blob/master/devicemotionevent.html
 [18]: /images/2011/07/c-rotation.png
 [19]: https://github.com/gre/same-game-gravity/blob/master/game.desktop.js#L31
 [20]: https://github.com/gre/same-game-gravity/blob/master/index.css#L35
 [21]: http://playframework.org/
 [22]: /images/2011/07/same_game_gravity_schema.jpg
 [23]: http://same.greweb.fr/public/javascripts/same.scores.js

see also [Same Game Gravity presentation][1].

2 years ago, I started to developed the [Same Game][2] as an HTML Canvas experiment. I’ve enjoyed developing this game, mostly because playing with HTML5 Canvas is so easy. Recently I’ve seen a nice increase in the user base (now around 250 visitors a day) – despite it being perhaps the simplest games I’ve ever developed. Simplicity is good, but my increase in users is thanks to the power of HTML5: The Same Game is available for 6 different platforms. And I can pump out new builds for them all in around 15 minutes. Here’s how…


**It’s often the simplest games which work. Too much complexity is not good.**

[  
![](/images/2011/07/gravity_exemple.png)
][2]

<!--more-->

In 2010, I learned how to make mobile web applications. It was also the year of the iPad. Out of interest I tried my same game canvas experiment on the iPad, and was surprised to find that it worked pretty well out of the box! Seeing it run on multiple devices was exciting – and the touch screens offered a new dimension for creating highly intuitive interactions. I mean, today, **even my mum can play Same Game Gravity without any help!** (That’s unfortunately not the case for her desktop)

That’s why I wanted to make Same Game for mobile. I started out developing and testing it as an Android application – because I have an Android phone. I created my own micro framework with some MVC concept (views, controllers, a router, etc.). The goal was to create **a simple and light web app that look like a native application**. For views? Portions of HTML. For transitions between views? CSS transitions. Supporting the “back” button of Android devices as a native application? I played with the hash (onhashchange event). 

In short, the web is wide and worldly enough to do pretty much everything you want with…

So I implemented the Same Game on Android. But (naturally) the game already existed on Android! I had to find something new! I was itching to fully exploit the possibilities of a new technology. Mobile has great potential – so it would be bad not to make use of new APIs. I discovered **the Accelerometer**. My idea was gravity: change the balls position by rotating the device.

But, many of my friends don’t have Android phones!

The Same Game Gravity is now available for iPad, iPhone, Android, Facebook, Chrome Store and desktop browsers. That’s a lot of platforms, with a lot of APIs to learn – and potentially a LOT of work in maintenance. But thankfully I didn’t have to go off learning Objective-C and Java Android, or keep track of arm-fulls of repositories! All the platforms are supported from **a single codebase**: thanks to the power and awesomeness of JavaScript, HTML, and CSS – combined with a nifty tool I developed [WebAppBuilder][4] to easily build each instance.


**I added a cool scoring system that spreads via multiple social networks simultaneously and easily – and now I have a truly cross-platform game!**

### The code

Desktop version source code is available on [Github][5].


#### The HTML

(see [game.html][6])


The HTML code is pretty simple.  
Basically, there is a **container which contains different**. Each section is a view of the game.

For instance here is the game view :  

```html
<section id="game">
  <div class="turnleft"></div>
  <div class="turnright"></div>
  <div class="gameStatus">
    <a class="i18n-back back" href="#!/">Back</a>
    <span class="timeline">
      <span class="remainingSeconds"><span class="remainingSecondsTime"></span> s</span>
    </span>
  </div>
  <div id="gameContainer">
    <canvas class="highlight" width="400" height="300"></canvas>
    <canvas class="main" width="400" height="300"></canvas>
  </div>
</section>
```

In the desktop version, **game.html** is wrapped into **index.html** in an iframe to keep the game independent of the context.

#### The CSS

(see [game.css][7])


CSS 3 is very rich.

CSS Transitions and CSS Transforms has been used to do view change.  

```css
/* Pre conditions :
 * With Javascript: 
   - A class "current" is setted for the current section view. 
   - All section after "current" must take a "after" class. 
 */
#main.enabletransition > section { /* #main must take "enabletransition" after the DOM load to avoid a first transition */
  transition-duration: 1s, 0s;  /* transform takes 1s duration, opacity doesn't have transition */
}
#main > section {
  z-index: -1;
  opacity: 0;
  transition-delay: 0s, 1s; /* opacity go to 0 in 1s */
  transition-property: transform, opacity;
  transform: translateY(-100%); /* go above the page */
}
#main > section.current {
  z-index: 0;
  opacity: 1;
  transition-delay: 0s, 0s;
  transition-transform: translateY(0%); /* go to the bottom */
}
#main > section.after { /* same as "#main > section.current ~ section" but without bugs */
  opacity: 0;
  transition-delay: 0s, 1s;
  transform: translateY(100%); /* go below the page */
}
 
/* Note that this is a part of the css code 
 * (you need to add -webkit-, -moz-, ... in some properties)
 */
```

#### The game core

(see [game.js][8])


Code is organized in different javascript “classes”.

The main components are :

*   [game.Grid][9] contains all the algorithm of the game.
*   [game.GameCanvasRenderer][10] is a game renderer (graphic part of the game) based on HTML Canvas element. It contains different functions called by **game.Game**.
*   [game.Game][11] contains all the game logic, the game loop and bind DOM events (touch, click, …).


#### game.desktop.js: a game instance for the desktop

(see [game.desktop.js][12])


This file contains all the specific code for the desktop version (it overrides existing classes). But it mainly contains the [game controller][13] handling different views and using all game classes.


##### Some significant code

```javascript
// Colors.get(nb) : pick nb random colors 
var Colors = function() {
  var clrs = [ new Color('#D34040'), new Color('#82D340'), new Color('#40C2D3'), new Color('#8B40D3'), new Color('#D3C840') ];
  return {
      get: function(nb) {
      return clrs
      .sort(function(){ return Math.random() - 0.5; })
      .slice(0, nb);
    }
  }
}();
 
/** Game instanciation **/
 
var gridSizeByDifficulty = [ // Size of grid for each difficulty
  {w:8, h:8},
  {w:12, h: 12},
  {w: 16, h: 16}
];
var colorNumberByDifficulty = [3, 4, 5]; // Nb of colors for each difficulty
 
var difficulty; // can be 0 (easy), 1 (normal) or 2 (hard)
currentGame = new game.Game({
  gridSize: gridSizeByDifficulty[difficulty],
  colors: colors=Colors.get(colorNumberByDifficulty[difficulty]),
  container: '#game', // container selector
  rendererClass: 'GameCanvasRenderer', // The class to use for rendering the game
  difficulty: difficulty,
  drawHover: true,
  globalTimer: new Timer().pause(),
  keepSquare: true // Keep a square ratio
});
```

### The gravity

The game gravity was maybe the hardest part of the game development. 

#### Using device Accelerometer for mobile/tablet version

I needed to find ways to access to the device accelerometer. For Android I used [PhoneGap Accelerometer][14]. But on iPhone I wasn’t able to get PhoneGap’s accelerometer.getCurrentAcceleration to work properly, so I used DeviceMotion event supported by iOS 4.2 . (see [DeviceOrientation spec][15]).

(A big thanks to [@42loops][16] for that: [devicemotionevent.html][17])


![Device orientation schema][18]


#### CSS Transforms and Transitions for the desktop version

Computers don’t have an Accelerometer. *Except maybe some macbook but I’m not sure people would like to turn macbook in 360°!* but the gravity concept is crucial to the game. I ended up implementing “gravity” via the arrow keys.  
The game is entirely rotated with [CSS Transforms][19] and animated with [CSS Transitions][20].


### The score system

I’ve written a web service with [Play! framework][21] to receive scores or retrieve them from Twitter, validate them and spread them with a json API and widgets.


![tweet example][22]


**This web service will be available soon for game developers.**

The power of this web service is the usage of **social networks**. It will retrieve peoples names, avatars, and their social links without needing to prompt the user.  
For game developers, social scores sharing is a nice way of **advertising your game**: someone shares his scores to his friends: so your game can spread virally.

See that little hash “$4f005″? That’s a way to check if sent scores are valid.  
In fact the web service allows you to handle your own security, via your own “twitter to scores” transformer. You can add a small Javascript function that is executed by the server when transforming a twit to scores – to ensure there hasn’t been any cheating.

The web service also provides **generic widgets** to easily embed game scores in websites.  
If a player has played a few different games using this web scores service, we can provide a “transversal” widget contains all scores of a player.

#### The Same Game Gravity Widget

Same Game Gravity use its own widget ([source code here][23]).

This widget is very customizable. Here’s an example of the code used to embed the widget anywhere (like in this blog post) :

```html
<script type="text/javascript" src="http://same.greweb.fr/public/javascripts/same.scores.js"></script>
<script type="text/javascript">
  new same.Scores({
    width: '250px',
    height: '400px',
    items: 3,
    period: 'all',
    platform: ['web', 'mobile', 'tablet'],
    type: ['hs_hard', 'hs_normal', 'hs_easy'],
    title: 'Best highscores ever',
    theme: {
      bg: 'rgb(218, 236, 244)',
      color: '#000',
      scores_bg: 'rgba(255, 255, 255, 0.5)',
      scores_color: 'rgba(0, 0, 0, 0.8)',
      link: '#1F98C7'
    }
  }).init().fetch();
</script>
```

### Conclusion

And here we are! 6 months to develop a game and release it on different platforms! I learn a lot about mobile development and I’m now more capable to develop other games.  
I learned that you should avoid using Canvas if you can use DOM instead because performance are bad on some mobile device whereas CSS Transitions / Animations are hardware accelerated.

Finally, I learned that game development is not only about programming! The marketing and the graphical parts are so important too.

Want to checkout the code or contribute to the game i18n? Just fork the [game repository][5].



#### Thanks

Big thanks to all game testers. Friends and colleagues, thank you very much!  
Special thanks to @mrspeaker for English help !

