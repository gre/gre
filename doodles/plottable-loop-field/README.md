# Plottable Field (8 frames)

> **Coming soon!** on fxhash.xyz – reserve will be for owners of "Plottable Wireframe".

'Field' explores the realm of noise fields. 13 unique kind of distributions. 16 possible inks. Different color combinations and animations.

A SVG file can be exported (right click save) to plot the 8 frames of this animation on a A4 paper. This collection is the first "plot loop" generator on a NFT marketplace.

You can enjoy the digital version that simulate ink effects and optional, thanks to the utility token decoupling, get a physical one: the digital NFT is the recipe to a plottable art piece, owning it confers the right to plot or request a physical plot from @greweb (https://greweb.me/plots/nft). @greweb would use fountain pens but other plotting artists are free to achieve it with their own materials and ship to collectors – as long as NFT is owned at request time.

## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: svg, noise, rand, byteorder, serde, serde_json, voronoi
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: React and GL-React.
