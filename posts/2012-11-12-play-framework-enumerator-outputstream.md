---
title: "Play Framework – Enumerator.outputStream"
description: "A few weeks ago, we’ve introduced a new feature in Play Framework: the Enumerator.outputStream method, allowing you to work with Java API requiring an OutputStream to generate content, for instance the java.util.zip API."
author: Gaetan
layout: post
permalink: /2012/11/play-framework-enumerator-outputstream/
tags:
  - playframework
  - iteratee
---

[1]: https://github.com/playframework/Play20/commit/0f1ec1479e490f2c8af4cd79dd0b6a14b0ea9f75
[2]: http://www.playframework.org/
[3]: http://mandubian.com/2012/08/27/understanding-play2-iteratees-for-normal-humans/

A few weeks ago, [we’ve introduced][1] a new feature in [Play Framework][2]: the `Enumerator.outputStream` method, allowing you to work with Java API requiring an `OutputStream` to generate content, for instance the `java.util.zip` API.

**Now, let’s see how easy it is to serve a big Zip generated on-the-fly without memory load with Play Framework.**

<!--more-->

## The Zip generation example

<script src="https://gist.github.com/4058734.js?file=Application.scala"></script>

This demo shows how to **generate a zip file on-the-fly** and directly **stream it** to an HTTP client **without loading it in memory or storing it in a file**.

It uses an `Enumerator` created with the `Enumerator.outputStream` method.  
The `OutputStream` provided by the method is then plugged to the Java’s `ZipOutputStream`.

For the example, we have generated a zip containing 100 text files, and each text files contains 100’000 random long numbers (yes, 100’000 !).

The zip size is approximatively 100 Mb. (and is generated in about 3Mb/s in my machine in localhost, but this can be improved)

The huge benefit of this is the download starts instantly, it means the Zip is streamed while it is generated.

## Show me the code!

Internally, it is implemented with a `Concurrent.unicast`, and a simple implementation of an `OutputStream` pushing into the unicast’s channel:

<script src="https://gist.github.com/4058734.js?file=Enumerator.scala"></script>

## About Iteratee and Enumerator

If you want to learn more about Iteratee concepts in Play Framework, I recommend you [this article][3].
