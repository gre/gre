# Plottable Wireframe (8 frames)

**Collection on https://www.fxhash.xyz/generative/13157**

More info: https://greweb.me/plots/509

'Wireframe' explores the realm of 3D noise mixed with various generative techniques. Many rare features and easter eggs are to discover.

A SVG file can be exported (right click save) to plot the 8 frames of this animation on a A4 paper. This collection is the first "plot loop" generator on a NFT marketplace.

You can enjoy the digital version that simulate ink effects and optional, thanks to the utility token decoupling, get a physical one: the digital NFT is the recipe to a plottable art piece, owning it confers the right to plot or request a physical plot from @greweb (https://greweb.me/plots/nft). @greweb would use fountain pens but other plotting artists are free to achieve it with their own materials and ship to collectors – as long as NFT is owned at request time.

<a href="https://greweb.me/plots/509"><img src="../../public/images/plots/509.gif" width="240" /></a><a href="https://greweb.me/plots/510"><img src="../../public/images/plots/510.gif" width="240" /></a><a href="https://greweb.me/plots/511"><img src="../../public/images/plots/511.gif" width="240" /></a><a href="https://greweb.me/plots/512"><img src="../../public/images/plots/512.gif" width="240" /></a><a href="https://greweb.me/plots/513"><img src="../../public/images/plots/513.gif" width="240" /></a><a href="https://greweb.me/plots/514"><img src="../../public/images/plots/514.gif" width="240" /></a><a href="https://greweb.me/plots/515"><img src="../../public/images/plots/515.gif" width="240" /></a>

## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: svg, noise, rand, byteorder, kiss3d, contour, geojson, serde, serde_json
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: React and GL-React.
