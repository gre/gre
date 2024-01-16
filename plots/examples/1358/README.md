---
date: "2024-01-15"
title: "1007 bytes of self-plottable code"
rootFile: m.mjs
image: /images/plots/1358.jpg
tweet: https://twitter.com/greweb/status/1746643014930292763
tags:
- genuary
- genuary24
---

This is a rework of [plots/1353](/plots/1353) 

---

In [plots/1353](/plots/1353) experiment, we have made JavaScript code that is "self-plottable": the code produces a plottable SVG of itself.

When working with pen plotters, we do not have fonts so we need to recreate the strokes to make letters and this code do it in the most minimal way possible: each character are encoded as few strokes on a 0..9 grid. The glyphs are placed on a A4 paper to recreate the code that produces them! 

This initial experiment was entirely made on Twitch, and the replay is available on Youtube: https://www.youtube.com/watch?v=C1z_4jzMDYk (the glyph editor, drawing the glyphs and making the code itself).

<iframe width="100%" height="360" src="http://www.youtube.com/embed/C1z_4jzMDYk?feature=player_embedded" frameborder="0" allowfullscreen></iframe>

---

for [Genuary 2024](https://genuary.art) 15 "Less than 1kb of artwork". It has been quite challenging to optimize the code to fit in 1007 bytes, but it was a lot of fun! Like "JS1K" kind of JavaScript golfing, you have to hack into the language features to make the worse possible hacks to save bytes. But here, it pushes this to the extreme: I had to consider removing as many characters as possible, so we got rid of these characters (vs the past version): `!$'*;?ABCI\bkuz{}` !!! You may think some part of the code are verbose but it's also all about removing these characters!

- no more `!` not operation. we wimake checks to be truthy only.
- no more `$` string interpolation. we use `+` instead.
- no more `'*` string multiplication. we use divisions instead!
- no more `;` statement separator. We don't use semicolons and otherwise we use `,` instead.
- no more `?` ternary operator. We use `&&` and `||` instead.
- no more `ABCI` string literals. These are due to `Array`, `viewBox`,.. we managed without!
- no more `\` escape character. We don't join on `\n` anymore, the stroke building is inlined into one path!
- no more `kuz` object literals. this came from `black` and other keywords, I took some hard decision to not even have `fill="none" stroke="black"` part, so you don't get a very beautiful preview but it's still plottable.
- no more `{}` object literals. This was one fun, we removed all blocks, no more `if` and `for`, and instead we use only `.map` and we embraced the `(instr1, instr2, ...)` synthax of JS https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Comma_operator
