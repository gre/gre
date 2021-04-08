---
title: 'PlayCLI: Play Iteratees + UNIX pipe'
description: PlayCLI is a new Scala library to work with UNIX commands and Play-Iteratees (a scala implementation of Iteratees facilitating the handling of data streams reactively)
author: Gaetan
layout: post
permalink: /2013/01/playcli-play-iteratees-unix-pipe/
tags:
  - iteratee
  - playframework
  - reactive
  - unix
  - library
---

 [1]: http://scala-lang.org
 [2]: http://www.playframework.org/documentation/2.0/Iteratees
 [3]: http://gre.github.io/playCLI-examples/api
 [4]: http://github.com/gre/playCLI
 [5]: http://github.com/gre/playCLI-examples
 [6]: /2012/08/zound-a-playframework-2-audio-streaming-experiment-using-iteratees/
 [7]: http://mandubian.com/2012/08/27/understanding-play2-iteratees-for-normal-humans/
 [8]: http://gre.github.io/playCLI-examples/api/#enumerate(command:scala.sys.process.ProcessBuilder,chunkSize:Int,terminateTimeout:Long)(implicitec:scala.concurrent.ExecutionContext):play.api.libs.iteratee.Enumerator[Array[Byte]]
 [9]: http://www.playframework.org/documentation/api/2.1-RC1/scala/index.html#play.api.libs.iteratee.Enumerator
 [10]: http://gre.github.io/playCLI-examples/api/#pipe(command:scala.sys.process.ProcessBuilder,chunkSize:Int,terminateTimeout:Long)(implicitec:scala.concurrent.ExecutionContext):play.api.libs.iteratee.Enumeratee[Array[Byte],Array[Byte]]
 [11]: http://www.playframework.org/documentation/api/2.1-RC1/scala/index.html#play.api.libs.iteratee.Enumeratee

 [12]: http://gre.github.io/playCLI-examples/api/#consume(command:scala.sys.process.ProcessBuilder,terminateTimeout:Long)(implicitec:scala.concurrent.ExecutionContext):play.api.libs.iteratee.Iteratee[Array[Byte],Int]
 [13]: http://www.playframework.org/documentation/api/2.1-RC1/scala/index.html#play.api.libs.iteratee.Iteratee
 [14]: http://www.scala-lang.org/api/current/index.html#scala.sys.process.package
 [15]: http://www.playframework.org/documentation/api/2.1-RC1/scala/index.html#play.api.libs.iteratee.Done$
 [16]: #pipe(command:scala.sys.process.ProcessBuilder,chunkSize:Int,terminateTimeout:Long)(implicitec:scala.concurrent.ExecutionContext):play.api.libs.iteratee.Enumeratee[Array[Byte],Array[Byte]]
 [17]: http://www.playframework.org/documentation/api/2.1-RC1/scala/index.html#play.api.libs.iteratee.Input$$EOF$


> **TL;DR.** PlayCLI is a new [Scala][1] library to work with UNIX commands and [Play-Iteratees][2] (a scala implementation of Iteratees facilitating the handling of data streams reactively). Here’s an overview:

<iframe src="http://gre.github.io/playCLI-examples/embedder.html#index.html" frameborder="0" width="550" height="452"></iframe>

## Links

*   [The scala API][3].
*   [PlayCLI source code (Github)][4].
*   [PlayCLI Examples application (Github)][5].

### SBT

```scala
"fr.greweb" %% "playcli" % "0.1"
```

<!--more-->

## Why PlayCLI

After having made [Zound][6] in a HackDay (an experiment to generate an audio stream with playframework iteratees and through the WAVE format), I figured out this was going to be hard to make it work with multiple audio format: *tell me if I’m wrong but*, there are not so much audio libraries in Java/Scala, or most of them does not support stream handling (and not reactively), and it was going to be crazy to re-implement everything in Scala (both in term of cost and performance).

Besides, **UNIX has plenty of tools** to do this and:

1.  they are **complete** and provide a lot of options
2.  they are **easy to use** (see how Bash is powerful as a consequence)
3.  Most of them **support streams** out of the box (via stdin / stdout)
4.  They are very **efficient** (written in C / assembly)

So why not re-use them from our reactive code?

### Similarities with UNIX pipes

> Take the expressivity of UNIX pipes, bring the power of Scala, mix it with Play Framework and you got a powerful framework for handling real-time and web streaming.

Play Iteratees are an elegant & powerful way to handle streams reactively, and I’ve actually always understood them like UNIX pipes, you have the same reactive code style: linearized declarative way of handling streams.

**Bash:**

```bash
cat words.txt | grep $word > result.txt
```

**Scala:**

```scala
Enumerator.fromFile("words.txt") &>   
  splitByNl &> // split a stream of Array[Byte] into stream of String (not impl here)  
  Enumeratee.filter(_.containsSlice(word))  |>>>   
  fileWriter // consume the steam while storing in a file (not impl here)
```

or if you prefer the “without symbol” version:

```scala
Enumerator.fromFile("words.txt").  
  through splitByNl.  
  through Enumeratee.filter(_.containsSlice(word)).  
  run fileWriter
```

However, It’s biased to say Iteratees are only UNIX pipes, they are more than that, but I’m not going to extend on that subject, they are at least statically typed and safe (it’s more than just a stream of bytes, see [this article][7]).

So if Iteratees are at least UNIX pipes, why can’t we use Unix pipes from iteratees?

**PlayCLI provides a bridge to use scala.sys.Process with play-iteratees.**

## More about PlayCLI

*(this is a copy of the API documentation)*

### Overview

Depending on your needs, you can **Enumerate / Pipe / Consume** an UNIX command:

[CLI.enumerate][8] is a way to create a stream from a command which **generates output**  
(it creates an [Enumerator][9][Array[Byte]] )

[CLI.pipe][10] is a way to pipe a command which **consumes input and generates output**  
(it creates an [Enumeratee][11][Array[Byte],Array[Byte]])

[CLI.consume][12] creates a process which **consumes a stream** – useful for side effect commands  
(it creates an [Iteratee][13][Array[Byte],Int])


#### Examples

```scala
import playcli._  
import scala.sys.process._  
  
// Some CLI use cases  
val tail = CLI.enumerate("tail -f /var/log/nginx/access.log")  
val grep = (word: String) => CLI.pipe(Seq("grep", word))  
val ffmpeg = CLI.pipe("ffmpeg -i pipe:0 ... pipe:1") // video processing  
val convert = CLI.pipe("convert - -colors 64 png:-") // color quantization  
  
// Some usage examples  
val sharedTail = Concurrent.broadcast(tail)  
Ok.stream(sharedTail).withHeaders(CONTENT_TYPE -> "text/plain") // Play framework  
  
val searchResult: Enumerator[String] = dictionaryEnumerator &> grep("able") &> aStringChunker  
  
Ok.stream(Enumerator.fromFile("image.jpg") &> convert).withHeaders(CONTENT_TYPE -> "image/png")  
  
Enumerator.fromFile("video.avi") &> ffmpeg &> ...
```

### Process

CLI uses [scala.sys.process][14]  
and create a Process instance for each UNIX command.

A CLI process is terminates when:

*   The command has end.
*   stdin and stdout is terminated.
*   [Done][15] is reached (for [enumerate][8] and [pipe][16]).
*   [EOF][17] is sent (for [pipe][10] and [consume][12]).

CLI still waits for the Process to terminate by asking the exit code (via `Process.exitCode()`).  
If the process is never ending during this phase, it will be killed when `terminateTimeout` is reached.

PS: Thanks to implicits, you can simply give a String or a Seq to the CLI.* functions a `ProcessBuilder`.

### Mutability

[enumerate][8] and [pipe][10] are **immutable**, in other words, re-usable  
(each result can be stored in a val and applied multiple times).  
**A new process is created for each re-use**.

[consume][12] is **mutable**, it should not be used multiple times: it targets side effect command.

### Logs

A “CLI” logger (logback) is used to log different information in different log levels:

*   **ERROR** would mean a CLI error (not used yet).
*   **INFO** used for the process’ stdout output of a [CLI.consume][12].
*   **DEBUG** used for the process life cycle (process creation, process termination, exit code).
*   **WARN** used for the process’ stderr output.
*   **TRACE** used for low level information (IO read/write). 

## Conclusion

I’m eager to see what you guys can do with such an API, it enables a lot of possibility, I’m especially thinking about multimedia purposes (using powerful commands like: ImageMagick, ffmpeg, sox,…).
