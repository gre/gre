---
title: 'Panzer Dragoon 1k'
description: 'Panzer Dragoon 1k is a 2D remake of Panzer Dragoon in 1k of JavaScript I made for JS1K 2014'
thumbnail: /images/2014/03/js1k.png
author: Gaetan
layout: post
tags:
 - gamedev
 - js1k
 - javascript
 - canvas
---

[demo]: http://js1k.com/2014-dragons/demo/1790
[source]: https://gist.github.com/gre/9504494
[jscrush]: http://www.iteral.com/jscrush/
[jscrush-npm]: https://github.com/gre/jscrush/
[demojs]: http://demojs.org/
[js1k]: http://js1k.com/
[p01]: http://www.p01.org/

[<img src="/images/2014/03/js1k.png" alt="" class="thumbnail-left" />][demo]

This article introduces my journey into the JS1K world
and a few tricks I've used for my entry ["Panzer Dragoon 1k"][demo] ([source][source]).

Welcome to the world of hacks, tricks and getting-things-done-at-any-price.
You will turn the worst JavaScript practices and ugliest JavaScript facts to your advantage.
Welcome to the world where coding the bad way is satisfying!

Panzer Dragoon 1k
---

- **[Play the JS1K entry][demo]**
- [Source Code][source]

Panzer Dragoon Original Game
---

<iframe width="640" height="480" src="//www.youtube.com/embed/peoRBj9U-jI" frameborder="0" allowfullscreen></iframe>

<!--more-->

JS1K
===

**[JS1K][js1k]** is a competition where you have to make a demo (or a game, or anything)
in less than **1 kilobytes** of JavaScript: **less than 1024 characters of source code**.

To reach that goal you will need ideas, JS ninja tricks 
and most important: **patience and perseverance**!
But really, **anyone can participate**.

I recommend that you take a look at [js1k.com][js1k] and browse the existing entries.
There is awesome guys participating to this yearly event,
I had the chance to meet some of them at [DemoJS 2013][demojs] Paris event.
Also checkout [www.p01.org][p01] which contains some very good examples of crazy short demos.

Tools
---

But first things first: you need tools to minimize source code (it can simply be removing comments and spaces, minifiers, or it can be much more crazy tools like crushers).

Personally, I'm using:

```bash
cat source.js | uglifyjs -c unused=false | tee minified.js | jscrush > crushed.js && wc -c *.js
```

This small homemade command (to use in a npm script) results for my game in:

```
    1019 crushed.js
    1756 minified.js
    7241 source.js
```

If you are interested, I've made this toolkit available in a complete boilerplate 
that you can easily fork for your own usage: 
[https://gist.github.com/gre/9364718](https://gist.github.com/gre/9364718).

> **P.S.** [`jscrush`][jscrush-npm] is a npm module that you can directly use from the CLI 
but it is a port of the awesome [www.iteral.com/jscrush/][jscrush].

The beginning: Saving bytes
===

It quickly becomes frustrating to compete in JS1K
because you are basically trying to put a cow in a car (or an elephant if you are ambitious!).
But this frustration actually becomes addictive!

**Saving bytes** is your job - once you get your first working prototype, and inevitably blow your byte limit.

When you reach that limit, a good idea is to practice an **"add feature -> remove code"** development loop 
that really makes you think hard about your ultimate goal, and helps improve your entry.

> **The JS1K-based development:**
> Adding more and more features,
> figuring out how to fill everything in,
> re-thinking your demo to only keep the essential features.
> This will keep making your demo better.

You have to make a very hard choice: ***Which feature to remove?***
It is all about budget, not in term of money but in term of bytes!
A bit like in daily life: making choices with limited resources!

JSCrush
---

***JSCrush*** is a crazy tool you may want to use to go deeper in the bytes reducing.

It basically implements a compression algorithm which is based on substring occurences.
The challenge of such a tool is not only to make a good compression but to make 
a very small decompressor embedded in the result code because 
this decompressor might be an overhead *(~ +60 bytes with small code)*.

If you are using *JSCrush* which I recommend for saving extra bytes,
you may want to use some tricks to go even further with it!

The first time, you usually can save about 20% of bytes with classic 1k minified code.
But if you optimize your code **for** *JSCrush*, you can save much more!
I've achieved about a 40% code reduction in my demo!

Most of the tricks is about finding code patterns (same succession of JavaScript source code characters) 
and trying to duplicate them.

When I say **duplicating code**, it is really about **DUPLICATING code!**

> Once "indexed", a duplicated code is likely to just take one more byte in the final crushed JavaScript!

Some tips and tricks
===

[<img src="/images/2014/03/js1k_2.png" alt="" class="thumbnail-right" />][demo]

This section will share with you a non-exhaustive list of tricks.
I'm not going to talk so much about the basic and classic ones, 
but a few novel ones that I found to work well in my entry.
You may prefer to directly read the [annotated source code of "Panzer Dragoon 1K"][source] instead!
Of course, most of those tricks work closely with the `| minify | jscrush` transformation.

> Careful! Some tricks might be counter-intuitive as first glance, 
> again it ties-in with the way JSCrush is working.

Reduce your language
---

All existing functions and properties are costly in bytes,
each time you use another one, it definitely add bytes.
To save bytes, you have to limit your set of functions/properties to use
or find ways to access them indirectly.

  - **Use as few variables as you can**: this is valid in computer science in general: the best systems are those with the fewest possible variables (states). if one variable can be computed out of others, it should be removed. Also consider allocating some temporary variables to re-use like in an assembly registry (e.g. `i`, `j`).
  - **Reduce the set of functions** you authorize yourself to use! You may just need to use "fillRect" for everything, or "arc". Also don't use both `Math.min` and `Math.max`, one can be implemented with the other.
  - **Minimize the different values / colors** you are using (most of the time digits are fine, but `#RGB` colors are costy).

Duplicated wins!
---

- Generally: try to **duplicate the exact same code everywhere**!

- **Do not use explicit aliasing** like `M=Math` and `C=M.cos`, JSCrush does that job for you.
- **Get rid of intermediary computation.** Prefer inline and duplicated computation over variable assigment.
- Also, `a*(b+c)` might be more bytes than `a*b+a*c` if `a` is an expression. (but doesn't work in all cases)

**A few examples:**

```javascript
a = b+c; translate(a, a); // NOPE!
translate(b+c, b+c); // YES!
```

```javascript
size = a+b+10; fillRect(x-size, y-size, 2*size, 2*size); // no please don't!
fillRect(x-(a+b+10), y-(a+b+10), 2*(a+b+10), 2*(a+b+10)); // YEAH!
```

```javascript
fillRect(10,10,20,20);
...
fillRect(9,9,18,18); // Can you afford to use 10,10,20,20 instead?
```

In my demo, I was able to factorize some code. For instance the way I draw and update the x,y of my opponents and particles are the same duplicate chunk of code:

```javascript
    bga();
    arc(
      // Update
      e[0] += e[3],
      e[1] += e[4],
      e[2],
      0, 9);
    fl();
```

- You sometimes can **save bytes by adding more code**! For instance, if you need `fillStyle` and `strokeStyle`, it may save bytes to always set both color at the same time! `fillStyle = strokeStyle = ...` even if you only need once.
- Always **use the same `function parameters`**. In my game, I use `function(e){` everywhere even if I don't use that `e` in all my functions. This is saving a bunch of bytes with JSCrush.
- **Here's a particularly crazy trick:** If you have different collections of complex objects, you can simply represent each item by a vector (an array) and figure out how you can make use the same indexes for the use-case.

In my game:

```javascript
o = []; // an opponent: [ 0: x, 1: y, 2: health, 3: vx, 4: vy, 5: locked, 6: hitTime ]
p = []; // a particule: [ 0: x, 1: y, 2: size,   3: vx, 4: vy, 5: damage ]
```

- You also may find better way of managing collections. Instead of using `t.push(o)` to add, `t.splice(i, 1)` to remove, and `for(i=0;e=o[i];i++){...}` to iterate. I am using `t[Math.random()]=o` to add, `delete t[i]` to remove and `for(i in o){ e=o[i]; ... }` to iterate. It saved a lot of bytes if you already use `Math.random()` somewhere else! For-in loops are also quite short and can by used for other tricks (e.g. *Programmatical aliasing*).

<blockquote class="twitter-tweet" lang="fr"><p>My <a href="https://twitter.com/search?q=%23js1k&amp;src=hash">#js1k</a> uses `t[Math.random()]=insert`, for-in loops and `delete t[i]` rather than push and splice. Saving bytes with jscrush</p>&mdash; Gaëtan Renaudeau (@greweb) <a href="https://twitter.com/greweb/statuses/439324052403277824">28 Février 2014</a></blockquote>


- Use just **one letter variable names** (mangling variables won't work because they are in window scope, and IMHO it is better for you to write them by hand)
- You will probably need to **initialize some variables**, but do it only if necessary (if you have `ReferenceError`) and **use the multi-assignment syntax**: `A = B = 0` if you can. You should never have constant variables, it saves bytes to directly use the value inline.
- **`with(c){ ... }`** in your main loop may save bytes. It makes all functions and properties of c (the drawing context) in the scope.

Language tricks
---
- **Never use `var`**, just put everything in `window`
- **Programmatically aliasing `c`'s method** may save you a lot of bytes (or may not, you have to check!). You also have to find the code which suit the best your use case. Be careful about collision. Here is mine: `for (e in c) c[e[0]+e[2]+(e[6]||"")] = c[e];`
- Do not waste ANY value returned by assignment and operators (i++, x=.., x+=...). I'm sure you can do it somewhere else!
Typical example:

```javascript
x += vx; y += vy; /* ... */ fillRect(x, y, s, s); // NOPE!
/* ... */ fillRect(x += vx, y += vy, s, s); // YES!
```

- Try to not separate update from drawing logic. Mixing them may save bytes.
- You don't want to use `addEventListener`, just define listeners straight on window! e.g. `onclick = function(){...`

Make your JS1K now!
---

[<img src="/images/2014/03/js1k_3.png" alt="" class="thumbnail-left" />][demo]

I'm really eager to see all JS1K entries 
because I usually enjoy reading people's code and 
especially all the crazy tricks that I can learn from your code :-)

This article was just sharing a bunch of tricks which work for my entry,
but you will find much better tricks for your demo -
so please do it and make your crazy work!


---
*Special thanks to [@mrspeaker](http://twitter.com/mrspeaker) for fixing my English*.
