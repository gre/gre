---
title: "Work in <progress />"
description: A progress tag will be displayed on recent browsers with a OS-native progress bar representing a loading.
thumbnail: /images/2012/04/progress_mac.png
author: Gaetan
layout: post
permalink: /2012/04/work-in-progress/
tags:
  - html
  - javascript
---

**Did you know browsers now have a built-in HTML tag for making progress bar?**

<progress style="width: 50%">(progress is not supported)</progress>

How cool is that!

It is perfect for making web applications loading bar in just one line of HTML and a few Javascript code.

A progress tag will be displayed on recent browsers with a OS-native progress bar representing a loading. Like many HTML tag, if it is not supported, it fallbacks nicely by displaying its inner content. This fallback content should either be your own designed progress bar or simply display a percentage.

It is today supported by Firefox 9 , Chrome, Opera and IE10.

<!--more-->

## Example

```html
<progress value="23" max="100">23 %</progress>
```

### On your browser:

<progress value="23" max="100">23 %</progress>

### On Linux / Firefox (with GNOME)

![](/images/2012/04/progress.png)

### On Mac OS / Chrome:

![](/images/2012/04/progress_mac.png)

### On IE 6:

![](/images/2012/04/progress_ie.png)

## Let’s see some cases:

### waiting

<progress max="1000"></progress>

```html
<progress max="1000"></progress>
```

### starting

<progress value="0" max="1000"></progress>

```html
<progress value="0" max="1000"></progress>
```

### in progress:

<progress value="500" max="1000"></progress>

```html
<progress value="500" max="1000"></progress>
```

### finished:

<progress value="1000" max="1000"></progress>

```html
<progress value="1000" max="1000"></progress>
```

## Making a download bar

When you need to load big resource like images, videos, or 3D materials, you usually want to display the progress of the download.  
You could still do it using some divs and CSS Javascript, but this is now much simpler to use a :

### One line of HTML:

```html
<progress id="download"></progress>
```

### And the Javascript:

(for more convenience, we are using jQuery)

```javascript
var totalBytes = 10000000; // CHANGE ME WITH THE SIZE OF THE RESOURCE
var req = new XMLHttpRequest();
var progress = $('#download');
progress.attr("max", totalBytes);
req.addEventListener("progress", function (e) {
  progress.attr("value", e.loaded).text(Math.floor(100*e.loaded/totalBytes) " %");
}, false);  
req.addEventListener("load", function (e) {
  // THE RESOURCE IS LOADED
  progress.replaceWith("Downloaded!");
});
req.open("GET","resource.dat",true);
req.send();
```

It is quite easy to extend my code to support multiple files to download.

It is also easy to use this progress bar for anything else, but remember it represents a progress. If you want to represent some kind of stats, refer to the dedicated tag.
