---
title: "“Plottable Mountain Moons” and an approach to query generative art"
thumbnail: /images/2021/12/plottable-mountain-moons/plot.jpg
description: "I plan to release a plottable generator every Wednesday – this article covers today's release & share some thoughts on how to approach generative art variety."
tags:
  - NFT
  - plot
---
[plotnft]: https://greweb.me/plots/nft
[fxhash]: https://www.fxhash.xyz/u/greweb

**=> https://www.fxhash.xyz/u/greweb <= is where you can find my plottable generators.**

I plan to release a plottable generator every Wednesday – this article covers today's release & share some thoughts on how to approach generative art variety.


**Last week highlight:**

<img width="40%" src="/images/2021/11/plottablestorm/digital.jpg" style="float: left">

[See article to explain this concept](/2021/11/plottable-storm) released last week with "Plottable Storm" from which I learned and grew some ideas for the next iterations!
Plottable Storm may still be available on https://www.fxhash.xyz/generative/1050 – it was released via a mint of 10tez, which was pretty expensive for the platform's current price average – but made sense as the first "plottable NFT" of the platform. Note that, getting a physical plot is only an option, on which I will ask (1) to own the NFT and (2) to pay extra fees for me to plot it on demand.

<br style="clear:both">

## 🎉 Release: Plottable Mountain Moons

Ok, this may be the best plot generator I have ever created so far. I have put so much effort. This is months of iterations and experiments concentrated into one generator. I had goosebumps exploring the results and plotting them. This is just an illegal amount of fun.

I have also improved the ink simulating shader and added animation to make it visually interesting on its digital rendered version. Here is a quick preview:

<video loop autoPlay muted src="/images/2021/12/plottable-mountain-moons/promo1.mp4"  width="100%"></video>

**Find it on => https://www.fxhash.xyz/u/greweb <=**

## Tips: Prototype, prototype, prototype

I've developed my generator in the mind that it would be realistic to plot with fountain pens, e.g. without destroying the paper. This means I had to test a bunch of prototypes to check that:
- Paper can handle it (think of it like "load testing" but with ink)
- It's good looking (ink need to react well, some ink mix can be bad looking for instance)
- that the digital preview and the ink simulation is accurate (I've added many new inks and I had to tweak colors)

Here are a few prototypes I did this weekend... Actually 18 plots! Beware I use that as an iteration loop, so some helped to tweak things and do not necessarily represent a possible outcome anymore, but it's mostly the case.

<img width="100%" src="/images/2021/12/plottable-mountain-moons/all.jpg">

Each plot is done on a 21 by 21 centimeters watercolor paper.

Here are a few zooms:

<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom1.jpg">
<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom2.jpg">
<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom3.jpg">
<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom4.jpg">
<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom5.jpg">
<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom6.jpg">
<img width="100%" src="/images/2021/12/plottable-mountain-moons/zoom7.jpg">

And finally, the OG one that is the generator cover:

<img width="100%" src="/images/2021/12/plottable-mountain-moons/plot.jpg">


## Economy: Supply and Price

Determining the supply and price to put an NFT generator has never been easy. In fxhash, there is a great average of price of 0.5tez to 2tez which is very cheap if you think about what is happening in Ethereum. In a way, it needs to fit in this platform range, but I also need to think a lot about the supply: to me, the supply is proportional to the variety of your generator, how different results it can produce. Low Supply creates scarcity while High Supply gives more chance to collectors and reduce the FOMO effect. The price is also a way to limit the number someone can take, but is not very fair for low budget,...

For my case, and it may only justify for my own plans. I fallback-ed a bit on the idea (at least this time) to make is a Low Supply.

I decided this time to make it high in supply and a relatively accessible price for these reasons:

- This generator covers a great amount of my plotting work this year. A lot of primitives and shape combination has been converged into one big generator. And I find it fair and ethical that I can't anymore in the future release new generators that overlap too much with these. So in a way, this is a sacrifice to make. That means, I better need the supply to be high enough to make the generator fully express its variety.
- The idea of this NFT, first of all which are pretty cool as digital art, is also to use them as a power to have a physical action: claiming a physical plot to be performed. That physical plot request costs a fee. (currently set at 15tez but may evolve depending on demand and my capacity to respond to it) That means I want the price of the NFT itself to be relatively accessible which means low, which means supply need to have a better offer to those who will want to find the rare gems.
- I want my collectors to fully embrace collecting for the art and lower the "collecting for the speculation". Increasing the supply will decrease the scarcity effect that exist on second market. That said, it's not that high supply (smolskull was 2000!) and there will be room for second market to occur for those who are looking for "the perfect plot to perform".

## Tech: In pursuit of the perfect rarity equilibrium

My plot generator are develop with Rust. I have developed myself my own set of tools and I will basically have a watch loop that recompile and regenerate a .svg each time I change code. My Visual Studio Code set up will have a SVG display within the IDE while I can edit the generative code. This makes a nice ~5s live reloading development loop. Not bad.

The problem of this is that it's a generator, it yields a lot of different results. So I usually would just have a "seed" parameter exposed on the CLI and generate a folder with tons of images. Not very practical neither.

This time was still a bit of this for the initial development. But at some point I have to really think about the rarity of the results and trying to make everything interesting and unique. on my JavaScript side, I have a `generateVariables(random)` function that takes a RNG and yield a bunch of parameters to give to my generator as well as all the "features" to put in the NFT metadata.

### rarity analysis

This was really useful to split this in a dedicated file, because I could then develop a small node script that would do an analysis on 100 000 tries and give me stats about rarity. For instance:

```
Ink Sherwood Green
                     undefined: 95.2%
                      Mountain: 2.1%
                         Stars: 2.1%
                          Both: 0.6%
Inks Count
                             2: 71.7%
                             1: 28.3%
Mountains 2-Colors
                            No: 80.6%
                           Yes: 19.4%
Mountains Visibility
                           Low: 46.7%
                          High: 34.8%
                          Full: 10.8%
                         Empty: 7.7%
```

Which is super useful to try to classify and adjust how much you want a given case to occur. Indeed it forces you to really think about your classification first.

### as a query language

Ok, now what's super cool about this is I have a way to query my generator to give me a result of a given query!! So I made it work. Here are few examples:

```
node script.mjs 'p["Primitives"] > 4 && p["Inks Count"]==2'
ooa7qfERP3s73oycm7hvvtexuidFMvHtcr6dcLRHSpM9Uz33gVt
...
node script.mjs '(p["Ink Aurora Borealis"] && p["Ink Poppy Red"]) && p["Primitives"]>4 && o.a1 > 0.5 && o.f1 > 0.02 && p["Elements Count"]=="Normal" && p["Mountains Visibility"]=="Low"'
ooAzT5R8UgCy7eFYbvtryykBt8AVVBf25MQG7eknVfzHstahomV
```

the query is literally JavaScript code. And here is the simple script to run it:

```js
const predicate = new Function("p,o", "return (" + (argv2 || "true") + ")");
let r, hash;
do {
  const { fxhash, random } = newRandom();
  r = generateVariables(random);
  hash = fxhash;
} while(!predicate(r.props, r.opts));
console.log(hash);
```

### generating the .svg files in a folder

I didn't plan to explain in depth my tech stack because it would deserve a whole article, but to summarize: **generator is developed in Rust language, compiled in WASM which takes props and outputs SVG, which is rendered with WebGL shaders (sorry for your headaches)**

WASM can run in a web browser and is pretty efficient. (I'm not sure you can say the same of p5js for instance)

What's cool is WASM can also run in Node.js. So putting together that `generateVariables(random)` and even the query system, I ended up with a very powerful tool.

For instance, here I could generate a lot of results that have Blue Mountain and either Yellow or Pink moons:

```
node script.mjs '(p["Ink Amber"] || p["Ink Pink"]) && p["Ink Bloody Brexit"]=="Mountain"'
```

<img width="100%" src="/images/2021/12/plottable-mountain-moons/folder.jpg">

Pretty cool hey!

and the JavaScript is faily simple evolution of the previous code:

```js
const wasmBuffer = fs.readFileSync(path.join(__dirname, "./rust/pkg/main_bg.wasm"));
const wasmLoad = import("./rust/pkg/main.mjs");
wasmLoad.then(async (wasmModule) => {
  await wasmModule.default(wasmBuffer);
  const predicate = new Function("p,o", "return (" + (argv2 || "true") + ")");
  let r, hash;
  while (true) {
    const { fxhash, random } = newRandom();
    r = generateVariables(random);
    hash = fxhash;
    if (predicate(r.props, r.opts)) {
      const svg = wasmModule.render(r.opts);
      fs.writeFileSync("dist/" + hash + ".svg", svg, "utf-8");
    }
  }
})
```

I will definitely reuse this idea in the future. **With powerful tools comes great responsibilities!**
