---
title: Automating Web App development for multiple platforms
description: In this article, we will explain why we’d choose web technologies to make applications and introduce WebAppBuilder, a tool to easily build different instances of an application. We’ll examine the Same Game Gravity as an example.
thumbnail: /images/2011/webappmaker.png
author: Gaetan
layout: post
permalink: /2011/06/automating-web-app-development-for-multiple-platforms/
tags:
  - linux
---

 [1]: /2011/06/automating-web-app-development-for-multiple-platforms/#webappbuilder
 [2]: http://gre.github.io/same-game-gravity
 [3]: https://github.com/gre/WebAppBuilder
 [4]: http://diveintohtml5.org/
 [6]: http://www.phonegap.com/
 [8]: http://mustache.github.com/
 [9]: http://sass-lang.com
 [10]: http://compass-style.org
 [12]: http://mrspeaker.net/
 [13]: https://github.com/jquery/jquery/tree/master/build


In this article, we will explain why we’d choose web technologies to make applications and introduce [**WebAppBuilder**][1], a tool to easily build different instances of an application. We’ll examine the [Same Game Gravity][2] as an example.


Using web to develop mobile applications is very **productive** and web technologies are **rich**.

[Fork WebAppBuilder on Github.][3]

<!--more-->

## Rich?

New web technologies have become rich with CSS3, HTML5 and new Javascript APIs are now being supported on most of smartphones. CSS3 animations, Web Service usage, local storage, Geolocation, drawing shapes (Canvas),.. are some example of new web features.

I won’t expand more on this topic but invite you to [visit this link][4] for more details.

## Productive?

Compared to a native application, the web application **paradigm is reversed**.  
The Web provide a common way to make applications.  
To develop a native application, you must adapt yourself for each device, each new API, or each new language,… but with Web, the device fits to you by proving bridges (accessible via JavaScript) to access device features!  
I mean, you don’t need to dive into the Java Android API or Objective-C language (for iPhone/iPad), or any other API for other devices… You just have to learn **web technologies**.

We are in 2011, the “only desktop application” model is over now, and mobile and tablet are two new platforms you should be aware of. So it changes everything about the technology to use.

### Having a common language for all instances

Instances of a single application can be numerous.  
In fact, an application can be projected in at least 3 axis of instance : The **platform** (mobile, tablet, desktop, …), the **Operating System** (Android, iPhone, webOS) and the **application version** (free version, full version, …).

![](/images/2011/application-axis3.png)

That’s pretty expensive to develop X instances of an application. This is a problem for developing the first version and mainly for maintainability : You want to fix bugs and add features once, and only once.

**So, the point is we need a common language to describe an application with multiple instances.**

### Web is great for that!

Computers have browsers, mobiles and tablets device have recent browsers.

To make your application development fully independent from the device, firstly you need a great **framework** to bridge your application and the device (like [PhoneGap][6]), secondly you need a great tool to easily **build** all applications from your common source code.


First of all, let’s see how to organize a web project.

### Good practice

This is how I’ve organize my project :

#### The source code directory

This directory contains all your web app source code. You should keep your application source code (with HTML, CSS, Javascripts, images, sounds, …) in one directory (like */src* ).  
You should **avoid specific code**, but sometimes you still need some specific behaviors for different devices. If so, I recommend you to put these differences inside different files (for exemple: *mobile.html*, *tablet.html*, *computer.html*,…).

#### Project skeletons

Keep one skeleton directory for each instance of your application.  
A skeleton directory will contains all the specific code related to the platform/device/version.  
**Frameworks like PhoneGap bring you these skeletons.**

* * *

## WebAppBuilder

![](/images/2011/webappmaker.png)

I created **WebAppBuilder : a lightweight Makefile to build your project**. This is a mashup of existing cool stuff like : a small template system (Mustache), SASS with Compass, Javascript minimizer, …

### Features of WebAppBuilder

*   Template easily your HTML files with [Mustache][8].
*   Copy, concatenate, minimize Javascripts however you want.
*   Retrieve Javascript files from URLs (useful for libraries).
*   Compile SASS files into CSS files (if you use [this awesome stylesheets language][9])
*   Support [Compass][10] if installed (you don’t need to provide it in your source, only an import works)
*   Merge your CSS files.
*   Copy and optionally rename resources you want to include (images, fonts, sounds,…).
*   Error handling and atomicity : if one operation fail, the make fail (javascript syntax error, sass syntax error, …)

You must have one Makefile per project skeleton, so you can easily define what to do with the */src* for the related platform/device/OS.

### Download or Contribute

[Fork me on Github][3]

### Example with my Same Game Gravity game

I developed these tools during the [Same Game Gravity][2] game development. 

A **make** inside my android/ skeleton gives me :

![](/images/2011/webappmaker-term.png)

And here is the Makefile I use :

#### Android Makefile

```makefile
# Same Game Gravity - Android full version #
        
        ###             ~ Web App Builder ~               ###
        #       a Makefile to compile a web project.        #
        #  designed for web project with different devices  #
        #  (mobile, tablet, desktop) but with common code.  #
        ###        by @greweb  -  http://greweb.fr/       ###
 
# BUILD_DIR : PATH to Web App Builder /build directory (the directory containing all build tools)
BUILD_DIR = ../build
 
# SRC_DIR : the source directory
SRC_DIR = ../app
 
# DIST_DIR : the dist directory (ex: assets for android, www for iphone)
DIST_DIR = assets
 
# RESOURCES : Your assets (images, sounds, fonts... and other static files)
# You can rename dist file by prefix newname= ( ex: index.html=iphone_version.html )
RESOURCES = Chewy.ttf logo.png background.jpg pop.mp3 swosh.mp3 gravity_exemple.png
 
# VIEWS : Views will be interpreted by Mustache.js
# You can pass arguments with JSON format.
# Example: index.html:"{key1:value1,key2:value2,...}"  <= no spaces!
VIEWS = index.html=mobile.html:"{versionType:'',version:'1.0',platform:'mobile',android:true,free:false}"
 
### SCRIPTS : all javascripts
# - You can pass an URL to retrieve
# - if you want to minimize the JS, prefix with '!'
# - to mix scripts, concat them with a comma ','
# - to set the destination name, you can prefix scripts with 'myname.js=' else the first script name is used ( exemple: all.js=util.js,ui.js,main.js ).
SCRIPTS = game.min.js=!game.js,!game.mobile.js,!md5.js \
          phonegap.min.js=!phonegap.js,!phonegap.webintent.js \
          jquery.min.js=http://ajax.googleapis.com/ajax/libs/jquery/1/jquery.min.js,jquery.ba-hashchange.min.js,jquery.tmpl.min.js
 
### STYLES : all styles : CSS or SASS
# - For .sass files, we compile them to css
# - Like before, you can mix styles with ',' and you can name your target by prefixing 'name='
STYLES = game.css=mobile.sass
 
########################################################################
 
 
all: welcome clean assets_views assets_scripts assets_styles assets_files
 
welcome:
	@@${BUILD_DIR}/welcome.sh
 
assets_base: 
	@@mkdir -p ${DIST_DIR}
 
assets_views: assets_base
	@@${BUILD_DIR}/compile_views.sh ${SRC_DIR} ${DIST_DIR} ${VIEWS}
 
assets_scripts: assets_base
	@@${BUILD_DIR}/compile_scripts.sh ${SRC_DIR} ${DIST_DIR} ${SCRIPTS}
 
assets_styles: assets_base
	@@${BUILD_DIR}/compile_styles.sh ${SRC_DIR} ${DIST_DIR} ${STYLES}
 
assets_files: assets_base
	@@${BUILD_DIR}/copy_resources.sh ${SRC_DIR} ${DIST_DIR} ${RESOURCES}
 
clean: 
	@@rm -rf ${DIST_DIR}
 
.PHONY: welcome clean assets_views assets_scripts assets_styles assets_files
```

### Configuring your IDE

I use mainly komodo and geany as an IDE. They both have a build system. I recommand you to configure your IDE to make && open the page just by pressing a shortcut key.

### Features planned

*   make should build the .apk for Android app

## Special thanks

*   to [mrspeaker][12] for English review.
*   to [jQuery build system][13] (js minifier)

