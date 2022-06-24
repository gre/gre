# Plottable Thousands

**Collection on https://www.fxhash.xyz/generative/15409**

'Thousands' explores the packing of recursive circles and golden angle spirals distributing various particles on a A4 canvas.

A SVG file can be exported (right click save) to plot the piece physically.

You can enjoy the digital version that simulate ink effects and optional, thanks to the utility token decoupling, get a physical one: the digital NFT is the recipe to a plottable art piece, owning it confers the right to plot or request a physical plot from @greweb (https://greweb.me/plots/nft). @greweb would use fountain pens but other plotting artists are free to achieve it with their own materials and ship to collectors – as long as NFT is owned at request time.

## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: svg, rand, byteorder, serde, serde_json
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: React and GL-React.

## Theorical variety

There are 9 different primitives that are used as basis for the shape. They are indeed organized in many different ways.

```
Primary Shape
                     Recursive: 58.0%
                          Line: 8.7%
                         Line2: 7.2%
                           Hex: 6.2%
                        Square: 4.8%
                          Circ: 4.6%
                           Tri: 3.8%
                          Plus: 3.6%
                   EmptyCircle: 3.1%
Second Shape
                     Recursive: 21.7%
                     undefined: 18.1%
                          Line: 12.2%
                         Line2: 10.5%
                        Square: 8.9%
                           Hex: 7.6%
                           Tri: 7.6%
                          Circ: 7.0%
                          Plus: 3.6%
                   EmptyCircle: 2.8%
```
