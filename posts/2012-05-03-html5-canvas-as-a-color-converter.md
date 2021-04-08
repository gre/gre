---
title: HTML5 Canvas as a color converter
author: Gaetan
description: We can implement an anythingToRGBA converter in 10 lines of Javascript!
thumbnail: /images/2012/05/color-alpha-options.png
layout: post
permalink: /2012/05/html5-canvas-as-a-color-converter/
tags:
  - canvas
  - javascript
  - color
---

 [2]: http://gre.github.io/illuminated.js/
 [3]: https://github.com/bgrins/spectrum
 [4]: http://www.w3.org/TR/css3-color/

<img src="/images/2012/05/color-alpha-options.png" alt="" class="thumbnail-left" />

I’m currently working on the User Interface of a scene editor for my [Illuminated.js library][2] with some color and alpha picker.

HTML5 now have the `<input type="color" />` and `<input type="range" />` which is nice. It works on Chrome and there are some [polyfills][3] to make it working on older browsers.

We will now see how we can easily **retrieve a rgba color from such an UI**, regardless of the color format given by the color picker and **combine the alpha component from the alpha range picker**.

> We can implement an **anythingToRGBA converter** in 10 lines of Javascript!

## What?

Basically, for instance, you have this: `"#ff6432"` and `0.8`

and you want this: `"rgba(255,100,50,0.8)"`

which is this color: <span style="background: #ff6432; display: inline-block; width: 50px">&nbsp;</span>.

> Well, of course, we could use a library with regexp parsers!

But there is a lot of different formats available especially if you want to convert a color from [CSS][4]!

Only for the <span style="color:blue">blue</span> color, you have at least 7 different representations: `#00F`, `#0000FF`, `rgb(0,0,255)`, `rgba(0,0,255,1)`, `hsl(255,100%,50%)`, `hsla(255,100%,50%,1)`,  
and… `blue`!

> Ouch, so let’s make a huge converter library!

Nope! 

All of these are color formats are supported by CSS and also Canvas.  
**So, why not just re-using what the browser can do?**

<!--more-->


## How?

Because we have access to Canvas in Javascript, **we can implement an anythingToRGBA converter in a few line of Javascript**:

```javascript
var getRGBA = (function(){  
  var canvas = document.createElement("canvas");  
  canvas.width = canvas.height = 1;  
  var ctx = canvas.getContext("2d");  
  return function (color, alpha) {  
    ctx.clearRect(,,1,1);  
    ctx.fillStyle = color;  
    ctx.fillRect(,,1,1);  
    var d = ctx.getImageData(,,1,1).data;  
    return 'rgba(' [ d[], d[1], d[2], alpha ] ')';  
  }  
}());
```

You have now a ready to use Javascript color library! 

`getRGBA("#ff6432", 0.8)` will returns `"rgba(255,100,50,0.8)"`.  
`getRGBA("red", 0.5)` will returns `"rgba(255,0,0,0.5)"`.

You can “standardize” your color and use it anywhere!

**Feel free to adapt the code to any other desired format.**

We can easily make the reverse (give a rgba color and get the #RRGGBB and alpha values):

```javascript
var extractColorAndAlpha = (function(){  
  var canvas = document.createElement("canvas");  
  canvas.width = canvas.height = 1;  
  var ctx = canvas.getContext("2d");  
  
  function toHex (value) {   
    var s = value.toString(16);   
    if(s.length==1) s = "0" s;  
    return s;  
  }  
  
  return function (color) {  
    ctx.clearRect(,,1,1);  
    ctx.fillStyle = color;  
    ctx.fillRect(,,1,1);  
    var d = ctx.getImageData(,,1,1).data;  
    return {  
      color: "#" toHex(d[]) toHex(d[1]) toHex(d[2]),  
      alpha: Math.round(1000*d[3]/255)/1000  
    };  
  }  
}());
```
