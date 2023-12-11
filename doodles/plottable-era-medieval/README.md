# Plottable Era: (II) Medieval

<!--
digital price: 80tz -> 20tz
redeem price: 80tz
tags: physical, phygital, irl, redeemables, plottable, rust, wasm, plot
-->

**Collection will be released soon on https://www.fxhash.xyz/u/greweb**

'Plottable Era: (II) Medieval' is the second generator in the "Era" generators series, highlighting different periods and civilizations. Following "Primitive", this new series shows epic battle in a medieval scenery with mountains, castles, and ships.

<!--
Each NFT is a recipe to a physical plot (A4) and utilizes the Redeemables feature of fxhash, allowing owners to unlock a physical plot directly coming from @greweb's pen plotter.

- TIER 1 plots are offered! (Redeem price will be refunded)
- TIER 2 can proceed to the Redeem with the physical cost price (paper, ink, shipping)

The NFT serves as the key to obtaining a physically plottable art piece. It exports an SVG file that can be used for plotting with pens. Plotter artists have the freedom to utilize their own materials and ship the completed artwork to collectors, provided they possess the NFT at the time of the request. To obtain the .SVG file, simply drag and drop it into a folder or right-click and save.

For those interested in the technical details, this work was created using WebGL + Rust + WASM and is licensed under CC BY-SA 4.0. Additional information and the source code for this piece can be found at this link: https://github.com/gre/gre/tree/master/doodles/plottable-era-medieval -->

## Technical notes

- project started in 2022, 18 months of work. countless hours of experiments and prototypes.
- procedural: Everything is made through code, using randomness. (no models, no texture, no image, only usage of a font for the text).
- complex variety: ~21000 lines of Rust code.
- efficient: ~200ms to generate a frame natively. ~500ms on WASM.

## Physical prototypes previews

When working on a generator, I need to test a lot of cases to ensure all cases are physical possible without pen plotter density issues. Here are some of them:

...

## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: `serde, serde_json, getrandom, instant, rand, fontdue, noise, image, bs58, base64`.
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: `REGL`.

## Features

### Landscape

### Elements

### Pen and papers

The generator uses a generative palette mapped to inks, ruled by the paper used underneath: `Black` paper will be used with gel pens, `Blue` paper will be used mainly with black and white and `White` paper is a regular paper to use with fountain pen inks. There are also some other rare paper cases.

```
TODO
```
