<!--
Plan:
Supply: 400
Price: 888 STARS
whitelist premint 50%
Allowlist to <insertaddress> to be able to mint 60 NFT during the "pre-mint" phase, before it is "public".
-->

# Plottable Golden Train

Each mint is uniquely created with code, generating a random digital variant of this golden train traveling in far west mountains. Art have many variations (bridges, cactus, clouds, eagles, colors,...). This artwork is also optionally physically plottable (pen plotter) as a A6-size postcard. Check the link if you also want a physical.

The NFT is the recipe to a physically plottable art piece, it exports a SVG that you can plot with pens. owning it confers the right to plot or request a physical plot from the artist (extra cost for the physical derivatives) using this link: https://greweb.me/plots/nft?stargaze
 – The artist would use fountain pens and gel pens to achieve the final plot, but other plotting artists are free to use their own materials and ship to collectors as long as the NFT is owned at the time of the request. Simply drag&drop (or right-click-save) in a folder to get the .SVG.

For those interested in the technical details, this work was created using WebGL + Rust + WASM and is licensed under CC BY-SA 4.0. Additional information and the source code for this piece can be found at this link: https://github.com/gre/gre/tree/master/doodles/plottable-golden-train –

Part of this collection was pre-minted in context of a collaboration with [Astroquirks](https://astroquirks.com/) validator.

## Released on... PublicWorks.Art -> Cosmos / Stargaze blockchain!

I've been releasing work in the past on Ethereum and Tezos. This time, I chose to try another platform and another blockchain. Cosmos is a very innovative blockchain and I'm happy to expand territories of my art minting the first plottable NFT on Cosmos / Stargaze blockchain.

My work will be released on https://publicworks.art/ which itself will allow to mint on https://www.stargaze.zone/

## Inspiration

This work is based on my own work, notably the Inktober exploration of last year: https://greweb.me/plots/707

## Physical protytypes previews

Many prototypes were made to reveal the generator as well as stress test its physical results.

![](./docs/20230129_230259.jpg)

## A cool digital display

As always, my work is also animated and the digital version accurately reproduces what could the physical look like.

![](./docs/digital.jpg)

https://youtu.be/FHd7OF-QVSY

## Theorical features

```
Bridge
                Regular Bridge: 39.0%
       Regular Reversed Bridge: 16.2%
          Regular Small Bridge: 15.3%
                 Double Bridge: 6.5%
 Regular Reversed Small Bridge: 6.4%
                Complex Bridge: 3.5%
        Double Reversed Bridge: 2.9%
           Double Small Bridge: 2.5%
            Regular Big Bridge: 2.0%
       Complex Reversed Bridge: 1.5%
  Double Reversed Small Bridge: 1.2%
          Complex Small Bridge: 1.2%
   Regular Reversed Big Bridge: 0.6%
 Complex Reversed Small Bridge: 0.6%
             Double Big Bridge: 0.5%
            Complex Big Bridge: 0.2%
    Double Reversed Big Bridge: 0.1%
   Complex Reversed Big Bridge: 0.1%
Cactus Density
                     undefined: 66.3%
                           Low: 31.9%
                          High: 1.7%
                       Extreme: 0.1%
Cloud Density
                           Low: 50.8%
                        Medium: 25.1%
                          High: 24.1%
                     undefined: 0.0%
Eagle Density
                           Low: 83.7%
                          High: 15.2%
                       Extreme: 1.1%
Gold Border
                     undefined: 92.7%
                           Yes: 7.3%
Inks
                  Black + Gold: 53.9%
                  Gold + White: 39.0%
                         Black: 3.0%
                         White: 2.0%
                          Gold: 2.0%
Inks Count
                             2: 92.9%
                             1: 7.1%
Mountain Kind
                             1: 20.5%
                             3: 20.2%
                     undefined: 20.1%
                             4: 19.7%
                             2: 19.5%
Paper
                         White: 53.8%
                         Black: 32.8%
                     Dark Blue: 7.2%
                          Grey: 6.1%
Precipice
                       Regular: 45.6%
                      Moderate: 27.7%
                          Deep: 23.0%
                     Very Deep: 3.6%
Train Slope
                          Flat: 44.2%
                        Gentle: 32.7%
                      Moderate: 17.8%
                         Steep: 5.4%
```

## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: svg, noise, rand, bs58, serde, instant, serde_json
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: React and GL-React.
