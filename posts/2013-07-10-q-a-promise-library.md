---
title: 'Qep1.: Q, a Promise library'
description: 'This article is the first of a series of small articles on the Q Javascript library and its eco-system. It is a brief introduction to Q Promises.'
thumbnail: /images/2013/07/promise_then_thumbnail.jpg
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
[1]: http://github.com/kriskowal/q
[2]: http://github.com/gre/qimage
[3]: https://github.com/kriskowal
[4]: http://wiki.commonjs.org/
[5]: http://domenic.me/2012/10/14/youre-missing-the-point-of-promises/
[6]: https://raw.github.com/kriskowal/q/master/design/README.js
[7]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/EventLoop
[8]: http://jquery.com/
[9]: http://wiki.commonjs.org/wiki/Promises/A
[10]: http://tritarget.org/blog/2012/11/28/the-pyramid-of-doom-a-javascript-style-trap/

# [A World Of Promises][0], episode 1

*This article is the first of a series of small articles 
on the Q Javascript library and its eco-system.
This article is a brief introduction to Q Promises.*

<img src="/images/2013/07/promise_then_thumbnail.jpg" alt="" class="thumbnail-left" />

[Q][1] is a **Promise library** in **Javascript** 
created 4 years ago by [Kris Kowal][3] who is one of the main contributor to [CommonJS][4]
where we can find the **[Promises/A][9]** specification.

[Q][1] is probably **the most mature and powerful Promise library in Javascript**
which inspired a lot of libraries like [jQuery][8].
It exposes a complete API with, in my humble opinion, 
good ideas like the separation of concerns between a "Deferred" object (the resolver) 
and a "Thenable" Promise (the read-only promise).

This article is a brief introduction to Q Promises with some examples.
For more information on the subject, I highly recommend reading
the article ["You're Missing the Point of Promises"][5] 
and [the Q implementation design README][6].

<!--more-->

## What is a Promise

A **Promise** is an object representing a **possible future value** which has 
a `then` method to access this value via callback. A Promise is initially 
in a *pending* state and is then either *fulfilled* with a value or *rejected* with an error.

![Schema representing Promise states: pending -> fulfilled|rejected](/images/2013/07/promise.png)
### Some properties

It is **immutable** because the Promise value never changes and each `then` creates a new Promise. 
As a consequence, one same Promise can be shared between different code.

It is **chainable** through the `then` method (and other Q shortcut methods),
which transforms a Promise into a new Promise without knowing what's inside.

It is **composable** because the `then` method will unify any Promise returned as 
a result of the callback with the current Promise (act like a map or flatmap). 
[Q][1] also has a `Q.all` helper for combining an Array of Promise into one big Promise.

## A solution against the [Pyramid of Doom][10] effect

*Javascript* is by nature an **asynchronous language** based on an [event loop][7] which enqueue events.
As a consequence, there is no way to block long actions (like Image Loading, ajax requests, other events), but everything is instead asynchronous:
Most of Javascript APIs are using **callbacks** - functions called when the event has succeeded.

**Problem with callbacks** is when you start having a lot of asynchronous actions.
It quickly becomes a [Callback Hell](http://callbackhell.com/).

### Example

Here is a simple illustration, let's say we have 2 functions, 
one for **retrieving some photos meta-infos from Flickr** with a search criteria: `getFlickrJson(search, callback)`, 
another for **loading an image from one photo meta-info**: `loadImage(json, callback)`. 
Of-course both functions are asynchonous so they need a callback to be called with a result.

With this callback approach, we can then write:

```javascript
// search photos for "Paris", load and display the first one
getFlickrJson("Paris", function (imagesMeta) {
  loadImage(imagesMeta[0], function (image) {
    displayImage(image);
  });
});
```
*(Imagine what it can look like with more nested steps.)*

> we can easily turn a *callback* API into a *Promise* API

#### Promise style

`getFlickrJson` and `loadImage` can now be rewritten as Promise APIs:

Each function has clean signatures:

* `getFlickrJson` is a `(search: String) => Promise[Array of ImageMeta]`.
* `loadImage` is a `(imageMeta: ImageMeta) => Promise[DOM Image]`.
* `displayImage` is a `(image: DOM Image) => undefined`.

...and are easily pluggable together:

```javascript
getFlickrJson("Paris")
  .then(function (imagesMeta) { return imagesMeta[0]; })
  .then(loadImage)
  .then(displayImage, displayError);
```

**This is much more flatten, concise, maintainable and beautiful!**

Note that if we want to be safer we can write:

```javascript
Q.fcall(getFlickrJson, "Paris")
  .then(function (imagesMeta) { return imagesMeta[0]; })
  .then(loadImage)
  .then(displayImage, displayError);
```

`Q.fcall` will call the function with the given parameters and ensure wrapping the returned value into a **Promise**.
So my code should continue working even if we change signatures to:

* `getFlickrJson` is a `(search: String) => Array of ImageMeta`.
* `loadImage` is a `(imageMeta: ImageMeta) => DOM Image`.

One other cool thing about this chain of Promises is **we can easily add more steps** between two `then` step, for instance a DOM animation, a little delay, etc.

### Error Handling

But a much important benefit is, unlike the callbacks approach,
we can properly **handle the error in one row** because one of the following steps eventually fails:

* `getFlickrJson` fails to perform the ajax request to retrieve the Flickr JSON data.
* The array returned by Flickr was empty so `loadImage` throws an exception.
* The `loadImage` fails (e.g. the image is unavailable).

This is called **propagation** and is exactly how **exceptions** work.

**Promise Error Handling** really looks like **Exception Handling**.

If it would be possible to have two methods:

* `getFlickrJsonSync` is a `(search: String) => Array of ImageMeta`.
* `loadImageSync` is a `(imageMeta: ImageMeta) => DOM Image`.

Then, the blocking code would look like this:

```javascript
try {
  var imagesMeta = getFlickrJsonSync("Paris")
  var firstImageMeta = imagesMeta[0]
  var image = loadImageSync(firstImageMeta)
  displayImage(image);
} catch (e) {
  displayError(e);
}
```

*...which is very close to Promise style.*

**Q Promises also unify Exceptions and Rejected Promises**:
throwing an exception in any Q callback will result in a rejected Promise.

```javascript
var safePromise = Q.fcall(function () {
  // following eventually throws an exception
  return JSON.parse(someUnsafeJsonString);
});
// safePromise is either fulfilled with a JSON Object
// or rejected with an error.
```

> Error handling with the callbacks approach is hell:

```javascript
getFlickrJson("Paris", function (imagesMeta) {
  if (imagesMeta.length == 0) {
    displayError();
  }
  else {
    loadImage(imagesMeta[0], function (image) {
      displayImage(image);
    }, displayError);
  }
}, displayError);
```


## Next episode

Next episode, we will show you how to create your own Promises with *Deferred* objects.
We will introduce **Qimage**, a simple Image loader wrapped with Q.

---

Special Kudos to <a href="http://twitter.com/42loops">@42loops</a> & <a href="http://twitter.com/bobylito">@bobylito</a>
for bringing Q in my developer life :-P
