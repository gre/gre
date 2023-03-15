---
date: "2023-02-26"
title: "5k followers of @greweb Twitter"
image: /images/plots/934.jpg
tweet: https://twitter.com/greweb/status/1629866413249757191
nft: https://objkt.com/asset/KT1FxuHijMyjUiHbAd9b3faaRUvo95oDnjM5/3
tags:
  - twitter
  - 70x50cm
  - letters
---


I just reached 5000 followers on Twitter and this is a physical celebration of this milestone. All Twitter handles were drawn on paper, with a pen plotter robot for 4 hours 18 minutes. a V5 black pen on light 70x50cm paper. There are about 55000 letters and this is approximatively 200 meters of ink.

**If you follow me on Twitter, you are very likely in this, will you spot yourself? Let me know on twitter. Good luck =)**

*(more zoomed photos at the end of the page)*

<iframe width="100%" height="400" src="https://www.youtube.com/embed/e5a05huOnyQ" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## Iterations

This idea was planned for a few weeks already and it has been more challenging that I thought to draw text with a plotter. I ended up developing my own letters system (started in [/plots/931](/plots/931) and improved in [/plots/932](/plots/932)).

It also took me many iterations to end up with the current fork of organized curves following the twitter logo. I had earlier iterations which only consisted of random grid packing. It is way more interesting to have organic curves and we can also pack more.

This work was simultaneously plotted on Twitch, Youtube and Twitter.

<iframe width="100%" height="400" src="https://www.youtube.com/embed/XyC4jAt2ZgI" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## Technical details

> Important: Everything is open source permissive license except the assets `letters.svg` that are my own very ugly letters. They are "All Right Reserved", feel free to inspire from them but please make your own. Thanks!

That said, it's not too complex to draw letters in SVG and connect them together with code. The source code is linked on top of this page and is available on Github. This video is also explaining it with a quick code review.

<iframe width="100%" height="400" src="https://www.youtube.com/embed/hTKT43DEPjQ" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

To summarize this:

- We pull 5000 followers from Twitter API and cache it locally in `followers.txt`
- We load `letters.svg` to be able to draw word curves.
- We make an image `twitter5k.png` to contains the shape we want to see appearing.
- From that image, we use [marching square](https://en.wikipedia.org/wiki/Marching_squares) and distance function to extract out isolines curves. We simplify them with [Ramer–Douglas–Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm) to avoid local noise and remove small curves and we cut them in segments where there are sharp edges. This is important to avoid any unreadable word to happen.
- We iterate on these segments to make words following them, we equilibrate an equal padding between words for the general density. We also use a collision mask to bail out any word that would be too close to another one.


<a href="/images/plots/934-z-1.jpg" target="_blank"><img src="/images/plots/934-z-1.jpg" width="100%"/></a><a href="/images/plots/934-z-2.jpg" target="_blank"><img src="/images/plots/934-z-2.jpg" width="100%"/></a><a href="/images/plots/934-z-3.jpg" target="_blank"><img src="/images/plots/934-z-3.jpg" width="100%"/></a><a href="/images/plots/934-z-4.jpg" target="_blank"><img src="/images/plots/934-z-4.jpg" width="100%"/></a>
