---
title: "CryptoAliens: Genesis [ethblock.art]"
description: "CryptoAliens are digital creatures generated from Ethereum blockchain blocks. They can be minted on ethblock.at by anyone, which establishes a limited set of CryptoAliens species. Each block produced on Ethereum have unique elements that can be visualized in creative ways."
tags:
  - NFT
  - shaders
---

[create]: https://ethblock.art/create/
[opensea]: https://opensea.io
[tech]: /2021/04/cryptoaliens-tech

> See also [CryptoAliens: Genesis, a technical look][tech].
>
> NB: screenshot available here are not representative of the actual EthBlock.art pieces. Due to performance, the real time NFTs are rendered with 128 pixels square resolution ([see technical article][tech]). **However, it's technically possible to recreate such high quality rendering and even high quality video!** It's just technically too slow for today's GPUs.

<img src="/images/posts/cryptoaliens/021.png" width="50%" /><img src="/images/posts/cryptoaliens/001.png" width="50%" />

> _From the most adorable to the creepiest, 'Genesis' establishes the first embryonic species of CryptoAliens. Which one are you going to chose? Each CryptoAliens creature gets born on an Ethereum block which nourishes its shape: the number of transactions gives more bones, total ETH value transferred gives weight, exceptionally big transfers gives big heads, it will be dark when born at UTC night time,... CryptoAliens also take their unique texture from Mandelglitch's blockstyle, therefore including the same rarity scheme. More generations of CryptoAliens will follow in future and may evolved from the chosen ones._

## What are CryptoAliens?

CryptoAliens are digital creatures generated from Ethereum blockchain blocks. They can be minted on [ethblock.art][create] by anyone, which establishes a limited set of CryptoAliens species. Each block produced on Ethereum have unique elements that can be visualized in creative ways.

<img src="/images/posts/cryptoaliens/014.png" width="50%" /><img src="/images/posts/cryptoaliens/017.png" width="50%" />

> **CryptoAliens are born and nourished from transactions, transactions are bones, ETH is flesh,... and many other aspects that this article will explain!**

<img src="/images/posts/cryptoaliens/032.png" width="50%" /><img src="/images/posts/cryptoaliens/013.png" width="50%" />

### So I decide which ones are the CryptoAliens?

**Yes! As a NFT minter, you are the creator and you contribute at establishing the first 'Genesis' series of CryptoAliens.** You decide which creature deserve to live. You are the curator and it is your responsible to do a lot of research and find the most adorable (or the creepiest?) creature!

<img src="/images/posts/cryptoaliens/036.png" width="50%" /><img src="/images/posts/cryptoaliens/004.png" width="50%" />

### How many CryptoAliens are there?

Every single CryptoAliens species is unique and there are currently 12 millions because that's as many blocks there are today (April 2021). Every 15 seconds, a new block is minted on Ethereum blockchain (with usually hundreds of transactions in it) making a new CryptoAliens possibility.
The block is the DNA, but the creature only starts existing when minted!

**TLDR. CryptoAliens only comes to life when someone mint it as an NFT on the [ethblock.art][create] contract.** They can then be sold and traded on [opensea.io][opensea]. There is **a limited amount of CryptoAliens possible to mint** so be wise at your choice. In this 'Genesis' series, the current supply is set to 100!

<img src="/images/posts/cryptoaliens/002.png" width="50%" /><img src="/images/posts/cryptoaliens/003.png" width="50%" />

### What is 'Genesis' series about?

The idea of 'Genesis' is that we are collectively going to create the initial species of this universe (with the NFT we chose to mint).

Minting a _CryptoAliens: Genesis_ specimen is giving birth to the creature, therefore the current NFT is visualized on [ethblock.art][create] as a video tape recording of that time of birth (with the block number, time, weight and number of bones). These data are included in the NFT itself and could be reused in future!

## What determines how a CryptoAliens specimen looks like?

There are many information contains in Ethereum blocks that will get used to determine the general shape and gives its rarity.

### Block's timestamp

<img src="/images/posts/cryptoaliens/040.png" width="50%" /><img src="/images/posts/cryptoaliens/041.png" width="50%" />

When the block happened during UTC night, the visual will be in dark mode.

### Block's transactions amount

<img src="/images/posts/cryptoaliens/007.png" width="50%" /><img src="/images/posts/cryptoaliens/008.png" width="50%" />

When a block contains a lot of transactions it will impact its general weight. It will be highlighted by this very heavy blobs shapes. That said, the weight can be more or less dense based on amount of bones and also unique for each CryptoAliens specimen.

### Block's heavy transfers in ETH

<img src="/images/posts/cryptoaliens/035.png" width="50%" /><img src="/images/posts/cryptoaliens/020.png" width="50%" />

As said in the introduction, "ETH is flesh". Even tho most of the time it will impact the general weight of the creature, when a block contains an expectionally high transfer of Ethereum value, it will be highlighted by a big "head" on the creature. ("head" in doublequote because none of our scientist really figured what is this)

### Block's exceptionally low amount of ETH transferred

<img src="/images/posts/cryptoaliens/005.png" width="50%" /><img src="/images/posts/cryptoaliens/006.png" width="50%" />

On the contrary, when a block contains almost no ETH transfers, the arms will be very thin. Clearly ETH traders didn't nourish enough this poor creature.

### Block's important ratio of gas used (vs ETH value transfer)

<img src="/images/posts/cryptoaliens/023.png" width="50%" /><img src="/images/posts/cryptoaliens/016.png" width="50%" />

It often appears in combination with the previous criteria, if the ratio `total gas / total eth transfers` is high, meaning that a lot of the ETH is into gas, there will be some blobs at the end of the arms.

### ...and more block rare features

There are a lot of special cases are rare conditions that can happen. I will not disclose and I will let you discover. Some are really rare and some will be discovered in the future (even the author of blockstyle won't be aware of all cases!).

### Block's hash

Finally, the block hash gives variety in the results. It's necessary in order to have truly unique 12 millions species. But it's only complementary to the various other criteria. There are many features that are getting impacted by it, including the skin texturing (see _Mandelglitch BlockStyle_ section).

<img src="/images/posts/cryptoaliens/043.png" width="50%" /><img src="/images/posts/cryptoaliens/042.png" width="50%" />
<img src="/images/posts/cryptoaliens/012.png" width="50%" /><img src="/images/posts/cryptoaliens/038.png" width="50%" />

### What controls does the creator have?

At creation time, the minter also have the ability to move a bit the specimen:

- `mod1` is a simple rotation around it.
- `mod2` is a simple climbing and zooming.
- `mod3` will flex a bit the shape to make it torn & twist a bit.
- `mod4` have an impact on the color palette scaling.

**On top of this, mods have the ability to transform the skin texturing** which is actually based on [Mandelglitch BlockStyle](https://ethblock.art/create/17)! That means the rarity elements of Mandelglitch are shared in this new BlockStyle.

### mmh, Mandelglitch BlockStyle?

<a href="https://ethblock.art/create/17"><img src="/images/posts/cryptoaliens/mandelglitch.png" width="10%" /></a> **[Mandelglitch](https://ethblock.art/create/17) is a BlockStyle on [ethblock.art](https://ethblock.art/create/17), derived from Mandelbrot fractal.**

The visibility of Mandelglitch on the skin has been intentionally contained, but sometimes it is more visible. Here are two examples:

<img src="/images/posts/cryptoaliens/031.png" width="50%" /><img src="/images/posts/cryptoaliens/022.png" width="50%" />

## ...and, What's next?

Who knows what's next! As everything is available on the blockchain, what you mint is saved immutably and forever. Me or other artists could fork the code ([available on Github](https://github.com/gre/shaderday.com/tree/master/blockarts/CryptoAliens)) to make animated version of the CryptoAliens that were chosen (as this code is open source). Also we can imagine doing crossover between species or doing "evolution" of these species over time. Everything is possible!

---

See also [CryptoAliens: Genesis, a technical look][tech].

My name is Gaëtan Renaudeau, and I'm a noise explorer. **feel free to ping me on Twitter [@greweb](https://twitter.com/greweb)**
