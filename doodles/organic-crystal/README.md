

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
