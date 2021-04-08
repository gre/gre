---
title: CSS-selector-based templating system for scalable JavaScript applications
description: In this article, we will focus on the power of CSS as a descriptive language, current template system approach and their problems with modularity and extensibility, and try to mix both features from the concept to a concrete implementation.
thumbnail: /images/2012/02/218px-Mir_diagram-fr.svg_.png
author: Gaetan
layout: post
permalink: /2012/02/css-selector-based-templating-example-with-javascript/
tags:
  - css
  - javascript
  - templating
---

 [2]: http://sliderjs.org/
 [3]: http://www.ubelly.com/2011/11/scalablejs/

<img src="/images/2012/02/218px-Mir_diagram-fr.svg_.png" alt="" class="thumbnail-left" />

In this article, we will focus on the power of **CSS as a descriptive language**, current template system approach and their problems with **modularity** and **extensibility**, and try to mix both features from the **concept** to a **concrete implementation**.

<!--more-->

<br style="clear: both" />

## What is CSS ?

CSS is an extremely powerful descriptive language.  
It helps to define **how to display a document** (e.g. a web page).

A style sheet contains a set of **CSS rules**.  
Each CSS rule has a **CSS selector** associated with a set of **declarations**.  
You can see a CSS selector as a selection filter applied on every HTML element. A few element can match a CSS selector if they fit the structure describes in this selector.  
Each **declaration** is composed of a couple (**property** : **value**).  
The **CSS property** is a predefined property related to a display or layout behavior.  
The **value** will apply a custom value for the property on all elements matching the CSS selector.

Let’s focus on some advantages of this descriptive language:

### A CSS rule is independant from others.

The order of CSS rules *(selector declarations)* **really** does not matter.  
The priority between CSS rules is based on the selector itself and not on their arrangement.

### You can “mix” CSS rule 
2 CSS rules can have the same CSS selector. An element can be matched with multiple CSS rules. CSS rules are merged, it’s called the cascading.  
This is the most important feature of CSS.  
It implies a very modular and extensible language.

#### Example

```css
a {   
  color: #33CC00;  
  text-decoration: none;  
}  
a:hover { text-decoration: underline; }  
#articles {  
  font-size: 12px;  
}  
#articles a {  
  color: red; /* overriding the generic color of a */  
}
```

## Some limitations of today’s template system

Most of template system are based on inherence between templates.

*   You have usually an “inclusion” approach: a template will “include” multiple external template. *(Many template into Many template)*
*   And an “extension” approach: You define in a main template an area where you can append a template. Others templates “extend” your main template. *(One main template for Many template)*

These approaches aims to factorize template codes and that’s great.

But it doesn’t fit my needs:

*   It brings **dependencies between templates**. 
*   If you add a new template, you have to modify existing templates.  
    If your application tend to go modules based, this is going to be unmaintainable. 

> **web application module (n)**  
> 1 : an independent unit of functionality that is part of the total structure of a web application 

### A solution for scalable applications and libraries

I’ve recently started to rewrite my [SliderJS][2] library and I needed to split it into very modular features and having loose coupling between each component.

I followed this [Scalable JavaScript Application Architecture][3] article.

So, **how to bring loose coupling in templating?**.  
Each module have its own template and know where to append. Bringing this logic in a main template would break the independency and if I need to add new modules soon, it will not working without modifiyng it.  
How to keep the scalability of the main template without modifying it?

The best solution I found is to combine **CSS selectors** concepts with **template system** approaches.

## CSS concepts applied on templating

I’ve decided to inspire from CSS: **attaching a CSS selector with a template**.  
It benefits from some CSS advantages explained before.

### Simple twitter widget example

```html
<div class="twitter">
  <h1>Twitter</h1>
  <ul class="twitts">
    <li class="twitt">Hello world!</li>
  </ul>
  <footer><a href="http://twitter.com/greweb">Follow me on twitter</a></footer>
</div>
```

#### Classical approach

The way to template it with the classical approach would be:

```html
<div class="twitter">
  <h1>Twitter</h1>
  <ul class="twitts">
    {for twitt in twitts}
    <li class="twitt">{twitt}</li>
    {/for}
  </ul>
  <footer><a href="http://twitter.com/{me}">Follow me on twitter</a></footer>
</div>
```

#### CSS selector based approach

But we can also split the original “template” in small fragments and mix all of them.  
It helps to define each templates independently.

**We can identify:**

- A **root template fragment**:

```html
<div class="twitter"></div>
```

- A **header fragment** appended with `.twitter` selector and with an **high priority**:

```html
<h1>Twitter</h1>
```

- A **twitts list fragment** appended with `.twitter` selector:

```html
<ul class="twitts">
  {for twitt in twitts}
  <li class="twitt">{twitt}</li>
  {/for}
</ul>
```

with **parameters** `twitts = [ "Hello world!" ]`

- An empty **footer fragment** appended with `.twitter` selector and with a **low** priority:

```html
<footer></footer>
```

- A “follow me” **link fragment** appended with `.twitter footer` selector:

```html
<a href="http://twitter.com/{me}">Follow me on twitter</a>
```

with **parameters** `me = "greweb"`

### Advanced example

This is another example with a slider. 

Let’s conceptually imagine the following template language:

```html
@root { html: <div class="slider"></div> }

.slider {
  html: <div class="slides"></div>
}

.slider div.slides {
  html: <canvas class="slides"></canvas>
}

.slider div.slides {
  html:
  <div class="slide">
    <a href="<%= link %>">
      <img src="<%= img %>" />
      <span class="caption"><%= title %></span>
    </a>
  </div>
}

.slider {
  priority: -10
  html: <div class="pager"></div>
}

.slider div.pager {
  priority: 10
  html: <a class="prevSlide" href="javascript:;">prev</a>
}

.slider div.pager {
  priority: -10
  html: <a class="nextSlide" href="javascript:;">next</a>
}

.slider div.pager {
  html: <span class="pages"></span>
}

.slider div.pager {
  html: <span class="pages">
    <% for (var i=0; i<slides.length; ++i) { %>
    <a class="page" href="javascript:;"><%= i+1 %></a>
    <% } %>
  </span>
}
```

combined with some parameters, it will result

```html
<div class="slider">
  <div class="slides">
    <div class="slide"><a href=".."><img src=".."/><span class="caption">...</span></a></div>
    <div class="slide">...</div>
    <div class="slide">...</div>
    <canvas class="slides"></canvas>
  </div>
  <div class="pager">
    <a href="javascript:;" class="prevSlide">prev</a>
    <div class="pages">
      <a href="javascript:;" class="page">1</a>
      <a href="javascript:;" class="page">2</a>
      <a href="javascript:;" class="page">3</a>
    </div>                                                                      
    <a href="javascript:;" class="nextSlide">next</a>
  </div>
</div>
```

Of-course we could also do this programmatically with DOM. But see the benefit of such a descriptive way to define things?

You should keep in mind that **the order of rules definition does not matter**. In that’s sense, it is **a mixable, extensible, modular and loosely-coupled template system**.


### More about this POC

Unlike CSS, **two same rules aren’t merged but are appended**.

The ***priority*** governs the order of append. The higher the value is, the sooner it is appended to the containers selected by the CSS selector.

As you can see, there is a **micro-templating** inside each rule. For this example, it looks like the John Resig ‘s Micro Templating.

Note also that a rule must be aware of its **parameters** to work properly. But this only concerns the implementation: You have to find a way to give a dynamic reference of these parameters when you add a rule.

### Concrete implementation

The code above was a conceptual proof of concept, but I implement a subset of these features in Javascript and made “SelectorTemplating.js” available here : <https://gist.github.com/1731611>

This is how it can be used for (almost) the same example. You will see different style of usage:

```javascript
var node = document.getElementById("slider");
var t = new SelectorTemplating(node);
var tmpl; // defined somewhere, the John Resig 's Micro Templating.

// root module
function root () { return '<div class="slider"></div>' }
t.add(null, root);

// slides module
var slidesTmpl = tmpl('<div class="slides"> <% if(obj.slides) { for(var i=0; i<slides.length; ++i) { var s = slides[i]; %> <div class="slide"> <a href="<%= s.link %>"> <img src="<%= s.img %>" /> <span class="caption"><%= s.title %></span> </a> </div> <% }} %> </div>')
var slides = [ ... ]; // mutable
t.add(".slider", function () { return slidesTmpl(slides: slides) });

// canvas module
t.add(".slider div.slides", function () { return '<canvas class="slides"></canvas>' });

// pager module
var pagesTmpl = tmpl('<div class="pager"> <span class="pages"> <% if(obj.slides) { for(var i=0; i<slides.length; ++i) { %> <a href="javascript:;" class="page"><%= i+1 %></a> <% }} %> </span> </div>');
var slides = [...]; // synchronised with the slides module
var prevButton, nextButton; // DOM element init when templated
var pages = function () {
  return pagesTmpl(slides: slides);
}
t.add(".sliderjs", pages, null, -10);
t.add(".sliderjs .options", function () { return '<a class="prevSlide" href="javascript:;">prev</a>' }, function (n) { prevButton = n[0] }, 10 }); // prepend first in options
t.add(".sliderjs .options", function () { return '<a class="nextSlide" href="javascript:;">next</a>' }, function (n) { nextButton = n[0] }, -10 }); // append at the end of options

// when all modules are init :
t.init();
```

```
t.add (selector, templateFunction, callback, priority)
 * selector is the selector function. if null, append to root.
 * a template function is an identifier in the template.
 * the callback is called at the end of the templating with 2 arguments : the appended nodes and the global container.
```

#### Algorithm of the template process

```
container := the container element
rules := an array containing all rules.
sort rules by priority.
(1) take one rule from rules
  - elements := []
  - if the selector is @root, elements := [container]
  - otherwise, elements := all elements which matches the selector
  - if the elements is empty, back to (1) by taking the next rule.
  - (2) if not, templatize the html and append it into all of these elements. remove the rule from rules. back to (1) by starting from the first rules. 

the loop (1) must end when :
  - there is no rules anymore
  - you have covered all the rules array without finding a match (without passing by (2) for this loop). In that case, it means some rules are not used.
```

There is a known limitation of the algorithm I intend to fix soon:
Once we found matching elements for a rule, we append the template in these elements once, and we remove the rule. It’s a simple way to avoid recursion. But this approach doesn’t work if a selector can potentially matches elements defined in different rules. **I know how to fix this but it’s not yet implemented.**

## What's next?

We are working hard for the next version (v2) of [SliderJS](http://sliderjs.org) by trying to make a revolutionary IDE platform for SliderJS. It requires a modulification of every components of SliderJS, we try to keep things simple (no external library required, the core system is only 4k sized). You will have more information soon!

This templating system should benefits of this work.

Keep in touch!
