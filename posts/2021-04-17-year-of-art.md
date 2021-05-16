---
title: "2021, the year of art"
thumbnail: /profile_plot.jpg
description: "2021 is a creativity reborn for me. Notes on what i've been doing the past months. Shaders, plots, NFTs,..."
tags:
  - plot
  - NFT
  - shaders
---

(TODO header on the blog post)

In this article, i'll do a tour of many things i've explored the past months: Shaders, Plots, NFTs,... Apologies for the long article, but it shows how crazy I have explored in the past weeks and months. Maybe 2 images summarize it all after all:

<img width="50%" src="/profile_laserblue.gif" /><img width="50%" src="/profile_plot.jpg" />

## Hey, it's @greweb, i'm back!

Welcome back to my blog. You may notice I've been silent here for a few years, no more blog posts, not a lot of creations or games made neither... well there is life as busy and passionate it can be: I've been continuing to learn Chinese, growing kids, doing archery, lot of gardening,... and work!

**The year 2021 is a reborn of creativity for me.** Despite the depressing pandemic context (we're getting bored at home), 2021 generally feels like the year of art. Maybe it's my bubble? my niche circle? the intersection of my crypto devs twitter sphere and demosceners/artists crypto sphere? Well, whatever this is, it has been incredible to see all these worlds colliding and converging to art around me. Specifically I've been working in the crypto currencies industry for the past 3 years now (as [Ledger Live](https://www.ledger.com/) tech lead), the market has been pretty bearish these past years but this year have bullrun and welcome a lot of artists jumping into the "crypto art" fun with NFTs.

**I'm going back to my origins!** As a kid, I always has been attracted by art, as a teenager I was in a drawing club, mainly [doing dry paster](https://twitter.com/greweb/status/1368570431029063684) which is pretty straightforward and interesting. A bit later, I fade away from drawing to computer. I was spending countless hours on Blender and 3D modeling, before I even was into programming. I was doing many visuals already –to impress a bit friends– without really knowing what "demoscene" was. Years later, as a developer, I've continued creating with code, challenging myself in many area, notably Canvas 2D and WebGL. I did [many mini games](https://greweb.itch.io/) over last years because it's such a great field to learn so many different skills. Life is a bit a cycle, now I really feel i'm closer than ever than my childhood, and still in pursuit to learn new things.

On these days, i'm a lucky creative coder, a weirdo noise explorer, spending hours exploring the beauty of perlin noise.

## Shader all the things

A few years ago, I've created and actively maintain [gl-react](https://gl-react-cookbook.surge.sh/) which brings OpenGL Shaders to the web and specifically to React. There are two goals in this library: making it easy for newlearners to get into shaders, making it very straightforward to implement image effects, possibility even without knowing how to implement a shader, if someone wrote that effect, it's just React component composition. I had the chance to work on this in the context of my work (back in the days at ProjectSeptember in New York, we had very specific needs).

Ending 2020, when France started its lockdown again I wanted to do something every day. I wanted to go back to write again some shaders, that's how I started "One day, One shader" – later renamed "shaderday". I managed to do this for about 60 days which is a great accomplishement. I will continue it on a less regular pace. My work is available on https://greweb.me/shaderday. This first series of days has been interesting to me as I rediscovered some minimalism as well as 3D raymarching techniques.

<img src="/images/2021/04/stripes.gif" width="33%" /><img src="/images/2021/04/chess.jpg" width="33%" /><img src="/images/2021/04/bank.jpg" width="33%" />

There are so many interesting creative coders in shaders space and I wouldn't be there without their incredible talent. I'm thinking about artists like [Inigo Quilez](https://www.iquilezles.org/) or [Patricio Gonzalez Vivo](https://thebookofshaders.com/) (to only quote two of them). Platforms like [Shadertoy](https://www.shadertoy.com/) are incredible resourceful if you like to "learn by code" like me.

## Plot all the things

For the new year 2021 I decided to learn Rust language and I needed a purpose to learn it. I followed a bit a twitter trend and decided to join the [#plottertwitter](https://twitter.com/search?q=%23plottertwitter) game. I initially did a poor hardware choice (I bought a cheap plotter on amazon) and was disappointed by its precision. I did more studies and felt on the AxiDraw choice which I don't regret. Very good material, it need some budget on the price (~500$) but definitely worth it.

Conceptually it's pretty simple: 2 axis (2 motors) and one "pen lifter", you put a pen in the holder and you instruct it to draw some strokes.

Practically it has been a great journey with a lot of things to learn on pens, papers and plotting tricks. At the same time I had to learn a totally new programming language. It was a fun fight to try to get things done and fight against the elements!

I still can't believe we can use fountain pens with this:

<img src="/images/2021/04/plot-parametric.gif" width="100%" />

### one plot per day, since January 1th, 2021

I usually will plot by batch but I managed to keep the pace of releasing one plot per day, and every day it's a new Rust script that gets published on https://github.com/gre/gre. I even did a few livestream on [Twitch](https://twitch.tv/greweb) which I paused a bit because it's hard to keep focus and performance.

I don't have a gallery on my website yet, but I used to put them all on a wall which I have early picture of that:

<img src="/images/2021/04/wall.jpg" width="100%" />

I stopped putting things on my wall because I have more creation that could fit on my wall now =D

For now, you can find them all on my homepage at [greweb.me](https://greweb.me) or in my [Twitter history](https://twitter.com/greweb).

## NFT all the things

Non Fungible Token. What a simple idea, but that fit so well with what crypto currencies have achieved so far. If you don't know what it is, a must much is [Andreas Antonopoulos's video](https://www.youtube.com/watch?v=Y-i9Jsm95ro). Ultimately, NFT attach data on a blockchain (mainly Ethereum, but we see emergence of alternatives), and most of today's NFT platform are a pointer to IPFS. An URL basically.

I keep asking myself: is it a bubble? what will remain out of this? I've tried a bunch of exploration on the topic. And as many things I do, I like to push to limits and go a bit meta.

### Where I come from: my work at Ledger

We've all heard about NFT a few years ago with concepts like [CryptoKitties](https://www.cryptokitties.co/), at least at Ledger it always has been a topic of curiosity, that and great tutorials like [CryptoZombies](https://cryptozombies.io/)... But this year the emergence of NFT is going crazily mainstream.

For me it really started on March 1th, where I started my own investigation of NFT platforms out there. I did this for curiosity, both creative and technical. But I also happen to work at Ledger, where we make this famous secured hardware wallet.

<img src="/images/2021/04/nanos.jpg" width="100%"/>

I wanted to study where are NFT really secured. Ledger have a good leadership on hardware wallet and generally a good integration in web apps, but in context of very young NFT platforms there are not a lot of guidance out there (I didn't see a lot of NFT tutorials to even mention the possibility to use a hardware wallet) and it is not even possible to use a Ledger device on some platform. The main status quo today is to use software wallets, usually browser extensions, and these [aren't necessary the most secured options.](https://www.ledger.com/academy/crypto/the-best-way-to-store-your-private-keys-hardware-wallets).

Some of these "crypto wallet browser extension" will have the option "Use a hardware wallet" but it's not really made first class citizen. Clearly, there are always room to improve and we plan to deliver something this year. I work in Ledger Live team and we are keen to address the UX in even better ways than what you can do today when it comes to dapps. I always see this the biggest challenge of Ledger: when it comes to a physical hardware wallet, how do you also try to bring the best user experience with it?

#### So what's the TLDR of the investigation?

What's sure is that Ethereum is a bit the king blockchain platform for NFT, and for Ethereum it usually falls down to MetaMask which does support Ledger hardware wallet. Now, and until we don't get ETH 2 or wide support for Layer 2 platforms, the sudden raise of fees (it costs a lot to do even regular actions on Ethereum) have turned many to new alternatives.

Alternative platforms I've personally explored (non exhaustive and personal):

- **Tezos** blockchain + hicetnunc.xyz – great support of Ledger hardware with Temple wallet (even tho you have to hack your way around to chose "use a hardware wallet", but like in Metamask)
- **WAVES** blockchain + sign-art.app – unfortunately no support of Ledger wallet on this dapp, but I think it's technically possible with current hardware and apps.
- **Polkadot / KodaDot** blockchain + kodadot.xyz – unfortunately no support of Ledger wallet at this stage. Something is broken on the "batch transactions" support.
- **Phantasma** blockchain + ghostmarket.io – unfortunately no support of Ledger hardware at this stage (not even in the hardware).

To summarized, only Ethereum and Tezos are currently really good in term of Ledger support at this stage.

### EthBlock.art: creative visualization of Ethereum blocks

I've found [ethblock.art](https://ethblock.art) idea brilliant and innovative. The way I see this is that it's a virtuous ecosystem, similar to [Supply Chain Transformation concepts](https://en.wikipedia.org/wiki/Value_chain) with a few actors, as explained here:

![](/images/posts/cryptoaliens/ethblockart.png)

I'm currently the author of 4 block styles but there are currently 21 other to discover at [ethblock.art/styles](https://ethblock.art/styles).

<img src="/images/blockstyles/17.jpg" width="25%"/><img src="/images/blockstyles/18.png" width="25%"/><img src="/images/blockstyles/19.png" width="25%"/><img src="/images/blockstyles/24.png" width="25%"/>

This is totally aligned to everything I like about generative art. I'm a creative coder, I write code for everything and I like this idea to split the concerns between the creative coder and the visual artist who will search a lot to find something they like.

### hicetnunc.xyz, the NFT hive of experiments

Tezos blockchain and specifically hicetnunc.xyz platform has become extremely popular among my artist sphere and I've also jumped into it.

I've started minting experiments from my previous work (shaders mostly, images and videos) but then, about one week after, someone figured out one could upload .SVG with JavaScript in it and literally have script that runs out of an NFT. It felt scary at first but they secured this in an `<iframe` with the `sandbox` mecanism that do not allows it to query the outside world.

Later on, they even made it more "first class citizen" and less hacky but the simple support of an HTML zip archive.

### NFT iframe, html, scripts,... Can we push it farther?

With a bit of curiosity and a tiny bit of troll/sarcasm/self-mockery of this whole thing. I decided to start a bunch of "concepts". Some art concepts, some are real thing that could be useful for some, in any case, i'm having fun and it's all that matters.

### Game as an NFT

On that weekend where the idea emerged, many of us made game as an NFT. I released 4 of our my previous games in NFT. It's convenient because in the past years, I've always made my game very minimalist in that they were usually for code golf competitions = basically they stick in very small amount of kilobytes, so they could easily fit in a .svg!

#### 'Panzer1k' 2d shooter

https://www.hicetnunc.xyz/objkt/9099

<img src="/images/2021/04/panzer1k.gif"/>

- 3d maze https://www.hicetnunc.xyz/objkt/13177
- ‘behind asteroids’ https://www.hicetnunc.xyz/objkt/9208
- world simulated with some ibex animals https://www.hicetnunc.xyz/objkt/11878

### General tools

#### Crypto Eyes

https://hicetnunc.xyz/objkt/36282

<blockquote class="twitter-tweet"><p lang="en" dir="ltr"><a href="https://t.co/wDkTgQipHh">https://t.co/wDkTgQipHh</a> v1.0.1 – fixed for GIF export on non square images. I&#39;ve sent the 2 buyers of yesterday a copy of this new version and i&#39;ve burned all the previous <a href="https://twitter.com/hashtag/nft?src=hash&amp;ref_src=twsrc%5Etfw">#nft</a> version. supply is lowered to 100. <a href="https://t.co/gpaIXGi7jn">pic.twitter.com/gpaIXGi7jn</a></p>&mdash; 𝗚aëtan 𝗥𝗘naudeau (@greweb) <a href="https://twitter.com/greweb/status/1383326182762975233?ref_src=twsrc%5Etfw">April 17, 2021</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

Some famous artist have even tried it!

<blockquote class="twitter-tweet"><p lang="und" dir="ltr"><a href="https://twitter.com/greweb?ref_src=twsrc%5Etfw">@greweb</a> :) <a href="https://t.co/xS59pbMVD1">pic.twitter.com/xS59pbMVD1</a></p>&mdash; ᴊᴏᴀɴɪᴇ ʟᴇᴍᴇʀᴄɪᴇʀ (@JoanieLemercier) <a href="https://twitter.com/JoanieLemercier/status/1383183295333163010?ref_src=twsrc%5Etfw">April 16, 2021</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>

### Plots and NFTs, why not both?!

Yes, more recently, and in recent celebration of my 100th day, I've actually put a lot of plot for sale as an NFT. The NFT acts as a 'proof of buy'.

I essentially put together 4 main collections out of my 100 first plots:

**They can still be found at [hic.link/greweb](https://hic.link/greweb) but I may burn they at some point by the way** (to expire them, the NFT not the original plot!)

## Future is great

At the end of the day, I always feel a lucky kid among big players, there are so many big artists and creative coders. This is very inspiring and always give me more ideas to see incredible work from other artists. It has been a fun playground to mint and buy other artists some NFTs. I feel very honored to have sell a bunch of NFTs, discussed with so many different people (in this lockdown context) and even sold 5 physical plots so far, using an NFT as a proof of buy. I believe this is one of the usecase that may remain on the long run, attaching NFT certificate to physical art and using it as a mean to trade.

I still have a lot to learn. I still have a lot of ideas to try out. I still have to find what is my art! But what a journey so far.
