## "Pattern 03" by greweb

<img src="images/001.png" width="400"/>

Pattern 03 is a tribute to fountain pen plotting. Using only line strokes, this generator composes many techniques into a unique script, lines follows various noise fields that generate many unique patterns. The BlockArt will be the main creation, but there will be a second life on the second market as these are physically plottable, the possibilities are immense: the NFT allows to download a SVG – recipe for a physical artist to perform the art with their own ways (@greweb or literally anyone in plotter community).

=== About the mods ===

MOD1: allows you to pick your color. which recreates the conditions of some inks. Black is very powerful but also other inks are very vivid. It's given to you as a mod control to pick one.

MOD2: is a controling two factors, the border width (0.5 to 1.0) and the intensity of the noise (cycles twice).

MOD3: is a general padding control which can also help adjusting border placement. It sometimes adjust lines distance too.

MOD4: both control the alignment of waves AND the amplitude of the "displacement" effect.

MOD5: controls the resolution of each line. (beware using high values slows down other mods).

MOD6: adds a variable blur effect.


=== Generation ===

As all BlockStyles, this script visualize Ethereum's block data: number of lines is driven by transactions count in the block, The shape is driven by $value of ERC20 transfers vs ETH. If most value is in ETH, a spiral is used, vertical lines means more $ in ETH, horizontal otherwise, cross hatch when there are exceptional ERC20 transfers. (evaluated with Aug 2021 static prices)

=== hints ===

There are many rare features to discover and intentionally not all documented: ultimately, don't hunt for the rarity that are "designed by the author", search for the organic rarity and enjoy patterns and unexpected shapes.


---


Notes:
- mod5 controls the resolution. Using a high value slow down other mods.
- This BlockStyle is a melting pot of technologies: Rust, WASM, SVG and WebGL!

WIP:
- the NFT traits are not finished.
- the SVG need a small optimisation to "dedup the moves".
- tweaks will continue to adjust the rarity features.
- Current BlockStyle image is not definitive.
- I plan to add a slight "paper" effect in post processing.

**options**

```
{"mod1":0.0001,"mod2":0.0001,"mod3":0.3,"mod4":0.5,"mod5":0.5,"mod6":0.3}
```

---

<img src="images/001.png" width="180"/><img src="images/002.png" width="180"/><img src="images/003.png" width="180"/><img src="images/004.png" width="180"/><img src="images/005.png" width="180"/><img src="images/06.png" width="180"/><img src="images/007.png" width="180"/><img src="images/008.png" width="180"/><img src="images/009.png" width="180"/><img src="images/010.png" width="180"/><img src="images/011.png" width="180"/><img src="images/012.png" width="180"/><img src="images/013.png" width="180"/><img src="images/014.png" width="180"/><img src="images/015.png" width="180"/><img src="images/016.png" width="180"/><img src="images/017.png" width="180"/><img src="images/018.png" width="180"/><img src="images/019.png" width="180"/>
