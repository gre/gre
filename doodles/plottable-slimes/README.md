# Plottable Slimes

**Collection on https://www.fxhash.xyz/generative/18109**

Packing of "Slimes" shapes on a A4 canvas. Variety of effects, ink and paper colors.

A SVG file can be exported (right click save) to plot the piece physically.

The digital NFT is the recipe to a plottable art piece, owning it confers the right to plot or request a physical plot from @greweb (https://greweb.me/plots/nft). @greweb would use fountain pens and gel pens but other plotting artists are free to achieve it with their own materials and ship to collectors – as long as NFT is owned at request time.

More details and Source code: https://github.com/gre/gre/tree/master/doodles/plottable-slimes

@greweb – 2022 – tech: WebGL + Rust + WASM – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/

## Physical protytypes previews

COMING SOON...

## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: svg, noise, rand, bs58, serde, instant, serde_json
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: React and GL-React.

## Theorical variety

```

Curving
                        Strict: 55.1%
                       Contour: 25.7%
                        Inside: 19.2%
Inks
             Black + Gel White: 19.7%
                     Gel White: 15.7%
          Gel Gold + Gel White: 9.1%
                         Black: 7.6%
                         Amber: 4.7%
                 Amber + Black: 3.9%
           Gel Red + Gel White: 3.5%
                  Black + Pink: 3.2%
                      Gel Gold: 3.0%
             Black + Poppy Red: 2.7%
                     Poppy Red: 2.5%
                 Bloody Brexit: 2.4%
                       Gel Red: 2.2%
                          Pink: 2.2%
          Gel Blue + Gel White: 2.1%
         Gel Green + Gel White: 1.3%
             Black + Hope Pink: 1.2%
                            ...
Inks Count
                             2: 54.7%
                             1: 45.3%
Intensity
                        Medium: 47.7%
                       Intense: 39.3%
                         Light: 9.5%
                       Extreme: 3.5%
Padding
                        Normal: 38.8%
                       Distant: 38.7%
                         Tight: 16.1%
                     undefined: 6.4%
Paper
                         black: 32.8%
                         white: 32.3%
                          blue: 19.9%
                           red: 15.0%
Shape
                         Slime: 56.2%
                  Smooth Slime: 24.8%
                          Snow: 7.7%
                Low-Poly Slime: 3.0%
                   Smooth Snow: 2.7%
                          Wind: 2.6%
         Smooth Low-Poly Slime: 1.6%
                   Smooth Wind: 1.4%
Size
                        Medium: 58.4%
                           Big: 22.2%
                         Small: 19.4%
Slimes
                          Many: 59.2%
                          Some: 27.2%
                         A lot: 7.2%
                           Few: 6.4%
```
