<!--

Price: 60->30->20->15 / 20mn
Supply: 128
Tags: printable, 3D, 3dmodel, 3dprint, stl, rust, wasm, physical
Flags: Interactive


Introducing 'Organic Crystal', a captivating collection of generative 3D objects that blur the lines between digital and physical art. This fxhash generator yield a great variety of abstract 3D shapes, from minimalistic statues, complex creatures to canyon sceneries.

Designed to be printed on a 3D printer, these organic shapes and forms will mesmerize you. The digital version mimic what can be obtained with Thermochromic PLA filament that changes color on different temperature. It adds an extra layer of depth to the pieces as the color of the filament changes from black to orange to yellow. Some other rare biomes may appear as well.

These NFT are digital first. Owning this NFT allows you to derive physical pieces. @greweb does not plan to print / ship physical pieces but is open to special requests via Twitter DM.

- Press 'S' to download a printable .STL, sized to a 6cm statuette. Usage of support is recommended.
- Press 'D' to switch between dark and light modes.

The source code for this piece is available on GitHub at https://github.com/gre/gre/tree/master/doodles/organic-crystal, and is licensed under CC BY-SA 4.0.

-->

## Organic Crystal

> by @greweb – 2023 – CC BY-SA 4.0 https://creativecommons.org/licenses/by-sa/4.0/

Organic Crystal is an art project that creates generative 3D objects that can be physically printed with a 3D printer. The project explores 3D low poly, smooth union skeletons of capsule structures to create organic shapes that resemble vegetals, rocks, and crystals. The visuals are designed for use with "thermochromic" print, using color changing PLA filament.

3D offers another dimension of generativity, I've used distance function paradigm to create triangles with marching squares. Various level of low poly is used and it can yields an infinite amount of variants, from minimalistic statue to a complex canyon scenery.

### Physical example

The visuals are designed accordingly to the usage of a "thermochromic" print, Notably using the color changing PLA filament black->orange->yellow (I have tried the one from brand TOPZEAL)

The generator will be available on the fxhash platform. @greweb does not plan to print / ship physical pieces but is open to special requests via Twitter DM.

### Tech

**Rust language for the generator. Libraries:**

- `stl` for serialisation to the 3D format
- `nalgebra` for geometry
- `isosurface` for the marching squares algorithm
- `chull` for Hull algorithm (used for the crystal)
- `bs58` and `rand` for RNG
- `serde` for JSON serialisation with the JS side

**JavaScript language for the UI. Libraries:**

- React
- Three.js
- react-three

> This work was started to enter "Maximalism" in Genuary.art, but I had so much fun to push this forward to a dedicated collection. Cheers, @greweb
