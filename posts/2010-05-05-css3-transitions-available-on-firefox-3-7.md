---
title: CSS3 Transitions
author: Gaetan
layout: post
permalink: /2010/05/css3-transitions-available-on-firefox-3-7/
tags:
  - css
  - animation
  - transition
---

 [1]: http://www.w3.org/TR/css3-transitions/

[CSS3 transitions][1] are now available on Firefox, Chrome, Safari, Opera and … IE9!, and it’s awesome.

> CSS Transitions allows property changes in CSS values to occur smoothly over a specified duration.

**Javascript is not anymore required for simple animation.**

Specifically, we don’t need Javascript to manage animation with **setInterval** or with any library like **jQuery.animate** : Forget the animation management, stay focused on the real work.

In this article, I will try to explain why and how using css3-transition with some examples.

<!--more-->

## Why ?

*   It’s **simple** and **smart** : adding one line of css code.
*   It **let the browser rendering animation instead of using javascript complex code**. You don’t have to worry about animation and performance. So you can stay focused on real part of your project.
*   It’s **cleaner** than some javascript animation implementation because you don’t modify style attribute during animations. So, it’s also probably more efficient.
*   **Degradation is great.** If your browser doesn’t support CSS Transition, it’s not really bad, only the animation is not available. **Other behaviors aren’t alterate**. For example, imagine a photo slide-show with zoom effect when changing photo. With an old browser, photos are just instantly zoomed without animation. That’s not bad.
*   And off course, It’s **standard**.

## How ?

CSS Transition are **extremely simply to use**.

### transition-duration

Basically, you set a **css time properties** in a css selector *like `-moz-transition-duration: 1s;` for mozilla*. This time define the animation duration. Browser will determine the transition between this selector and a descendant selector.

Not that css3 transition is currently in draft mode, so there are multiple property for each browser (the prefix change). 

For Firefox (3.7 ), Chrome (and other webkit browser) and Opera, you have to use : 

```css
-moz-transition-duration: 1s;  
-webkit-transition-duration: 1s;  
-o-transition-duration: 1s;
```

Don’t panic, in the future (on CSS3 release), only one property will be used.

#### Example

```css
.box {  
  -moz-transition-duration: 1s;  
  -webkit-transition-duration: 1s;  
  -o-transition-duration: 1s;  
  
  margin: 10px;  
  background-color: red;  
}  
.box:hover {  
  margin: 50px;  
  background-color: green;  
}
```

On mouse over the **.box**, during one second : margin will move from **10px** to **50px** and background-color will move from **red** to **green**. That’s all!

### transition-property

You can also specify the name of the CSS property to which the transition is applied.

For instance, **color**, **width**, **opacity**, …

#### Like this

```css
-moz-transition-property: margin, background-color;  
-webkit-transition-property: margin, background-color;  
-o-transition-property: margin, background-color;
```

### Others properties

More properties are available to specify more deeply transition effects. Retrieve them on [CSS Transitions Working Draft][1].

*   transition-timing-function
*   transition-delay

## Examples

Here are some CSS transition examples.

* [A box with color, text, shapes transformation.](/demo/css3/transition/box1/)
* [Letters animation.](/demo/css3/transition/letters/)
* [Image slider.](http://sliderjs.org/)
* [Navigation bar.](/demo/css3/transition/navbar/)

