# Plottable Era: (II) Medieval

**The collection has been released on https://www.fxhash.xyz/u/greweb**

'Plottable Era: (II) Medieval' is the second generator in the "Era" generators series, highlighting different periods and civilizations. Following "Primitive", this new series shows epic battle in a medieval scenery with mountains, castles, and ships.

## Technical notes

- project started in 2022, 18 months of work. countless hours of experiments and prototypes.
- procedural: Everything is made through code, using randomness. (no models, no texture, no image, only usage of a font for the text).
- complex variety: ~21000 lines of Rust code.
- efficient: ~200ms to generate a frame natively. ~500ms on WASM.

## Physical prototypes previews

When working on a generator, I need to test a lot of cases to ensure all cases are physical possible without pen plotter density issues. Here are some of them:

<a href="https://greweb.me/plots/1318"><img src="https://greweb.me/images/plots/1318-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1319"><img src="https://greweb.me/images/plots/1319-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1320"><img src="https://greweb.me/images/plots/1320-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1321"><img src="https://greweb.me/images/plots/1321-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1322"><img src="https://greweb.me/images/plots/1322-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1323"><img src="https://greweb.me/images/plots/1323-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1324"><img src="https://greweb.me/images/plots/1324-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1325"><img src="https://greweb.me/images/plots/1325-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1326"><img src="https://greweb.me/images/plots/1326-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1327"><img src="https://greweb.me/images/plots/1327-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1328"><img src="https://greweb.me/images/plots/1328-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1329"><img src="https://greweb.me/images/plots/1329-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1330"><img src="https://greweb.me/images/plots/1330-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1331"><img src="https://greweb.me/images/plots/1331-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1332"><img src="https://greweb.me/images/plots/1332-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1333"><img src="https://greweb.me/images/plots/1333-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1334"><img src="https://greweb.me/images/plots/1334-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1335"><img src="https://greweb.me/images/plots/1335-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1336"><img src="https://greweb.me/images/plots/1336-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1337"><img src="https://greweb.me/images/plots/1337-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1338"><img src="https://greweb.me/images/plots/1338-thumbnail.jpg"/></a><a href="https://greweb.me/plots/1339"><img src="https://greweb.me/images/plots/1339-thumbnail.jpg"/></a>


## License

CC BY-NC-ND 4.0

## Technical stack

- [lib.rs](./rust/src/lib.rs) Rust for the generative art logic and SVG generation. Libraries: `serde, serde_json, getrandom, instant, rand, fontdue, noise, image, bs58, base64`.
- [index.js](./index.js) WASM + WebGL for the frontend rendering. Libraries: `REGL`.

## Features

These are the public "features" of the generator and their theorical probability of occurence.

### Castle

```
Castle
	77%: "Regular"
	20%: "On The Sea"
	3%: "Huge"
```

### Day Time

```
Day Time
	73%: "Day"
	27%: "Night"
```

### Specials

Specials are special events that can happen in the scene. They are not mutually exclusive.

```
specials
	88.5%: (none)
	2.3%: "Barricades"
	1.7%: "Dragon"
	1.3%: "Trebuchets"
	1.2%: "Chinese"
	0.9%: "Cyclopes"
	0.9%: "Excalibur"
	0.7%: "Sauroned"
	0.7%: "Montmirail"
	0.6%: "TrojanHorse"
	0.6%: "EaglesAttack"
	0.4%: "Sandbox"
...sub 0.1% to get any combination of 2 or more specials.
```

### Pen and papers

The generator uses a generative palette mapped to inks, ruled by the paper used underneath: `Black` paper will be used with gel pens, `Blue` paper will be used mainly with black and white and `White` paper is a regular paper to use with fountain pen inks. There are also some other rare paper cases.


```
Paper
	37%: "White"
	36%: "Black"
	17%: "Dark Blue"
	7%: "Blue"
	2%: "Grey"
	1%: "Red"
```

```
Inks Count
	50%: 3
	34%: 2
	16%: 1
```

```
Inks
	16%: "Gold Gel, White Gel"
	14%: "White Gel"
	9%: "Black, White Gel"
	7%: "Gold Gel, Silver Gel, White Gel"
	7%: "Gold Gel, Red Gel, White Gel"
	5%: "Amber, Black, Poppy Red"
	4%: "Amber, Black, Sailor Sei-boku"
	3%: "Amber, Black, Soft Mint"
	2%: "Silver Gel, White Gel"
	2%: "Red Gel, White Gel"
	1%: "Black"
	1%: "Black, Poppy Red"
	1%: "Amber, Black"
	1%: "Amber, Black, Spring Green"
	1%: "Amber, Aurora Borealis, Black"
	1%: "Black, Poppy Red, Sailor Sei-boku"
	1%: "Blue Gel, Gold Gel, White Gel"
...many others
```
