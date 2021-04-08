- tezos plot
- bucket + paint. rec tree?
- LIVE: peinture + 3 couleurs en ligne et triangle spiral
- LIVE: recursive trees. then maybe around a center. have slight variation to make unique branches.
- instead of voronoi. use another way to group samples. K-means?
- image based vector field. for each point, we infer the vector field from an image regarding how similar the color is. use granularity and interpolation for perf. throw N rays around the pixel and look up a few pixels in these various direction (need interp). find the angle that minimize the sum of euclidian distances of colors (query by the ray from original point). we might need some gaussian or have a blurry image to make sure the vecotr influence each other.
- impl an algo that will try to avoid other lines/ make them parallel.
- LIVE: CMYK spiral
- utility fn to preserve image ratio
- reuse the collision lines to actually run them in parallel. try on an aggressive vector field.
- plot ortho rays from diff angle&color and run them against 2D objects using distance function
- noise holes in a vector field with line stops system.
- combine sampling + vector field
- maze (find something unique, not done by anyone)
- use word as a "no drawing" rule on a vector field
- combine vector field in background and voronoi polygons in foreground.
- figure out how to make curves instead of lines
- top-down sorting to connect the dots.
- TSP with multiple sampling
- hatching 3D obj with spiral to make the fill (if inside the polygon)
- have a simple extended shape that would repeat X times and we would use many colors
- triangle recursion

- diff materials
- using https://docs.rs/rust-3d/ to make some 3D views?

Prime GamingMrHalzy: ttf_parser gives drawing instructions
Prime GamingMrHalzy: and then there are some tessellation crates (lyon is used by Servo)
