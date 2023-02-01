## Plottable Sliced Spiral

**=> https://www.fxhash.xyz/generative/24533 <=**

Celebrating Genuary 2023 with this collection of only 32 items. As part of Genuary #31, I revisited spiral strokes exploration (https://greweb.me/plots/636) and mixed it with another slicing study (https://greweb.me/plots/823). Can't wait to see what unique pieces will emerge among the 32. From minimalistic to destructed spiral, various spins and rare color cases.

🌀

The NFT is the recipe to a physically plottable art piece, it exports a SVG that you can plot with pens. Owning that NFT gives you the right to plot the piece yourself or have the artist plot it for you (additional cost for the physical copy) using this link: https://greweb.me/plots/nft
 – The artist would use fountain pens and gel pens to achieve the final plot, but other plotting artists are free to use their own materials and ship to collectors as long as the NFT is owned at the time of the request. Just drag and drop (or right-click and save) the file into a folder to obtain the .SVG

🌀

For those interested in the technical details, this work was created using WebGL + Rust + WASM and is licensed under CC BY-SA 4.0. Additional information and the source code for this piece can be found at this link: https://github.com/gre/gre/tree/master/doodles/plottable-sliced-spiral

### Theorical rarity

The collection will be very small (32 items) so there is a high chance some of these features will never happen, but here is a statistical distribution:

```
Splits distribution:
    High                           : 48%
    Low                            : 28%
    Medium                         : 22%
spins distribution:
    Medium                         : 42%
    Low                            : 41%
    High                           : 15%
axes distribution:
    Two                            : 27%
    Three                          : 22%
    Four                           : 20%
    One                            : 14%
    Five                           : 11%
    Many                           : 4%
Sliding distribution:
    Low                            : 25%
    Medium                         : 24%
    High                           : 23%
    Extreme                        : 19%
    None                           : 6%
Inks distribution:
    Gold Gel, White Gel            : 44%
    Sailor Sei-boku, iroshizuku ina-ho : 13%
    White Gel                      : 11%
    Blue Gel, White Gel            : 11%
    Amber, Hope Pink               : 7%
    Sailor Sei-boku                : 2%
    Amber, Black                   : 2%
    Gold Gel                       : 1%
    Amber                          : 1%
Inks count distribution:
    2                              : 80%
    1                              : 19%
Paper distribution:
    Dark Blue                      : 32%
    White                          : 30%
    Black                          : 22%
    Red                            : 14%
```


## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: svg, noise, rand, bs58, serde, instant, serde_json
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: React and GL-React.