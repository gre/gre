# CryptoAliens: Genesis

## Ok, so how is it implemented technically?

First of all, I would like to point out the [source code is available here on Github](https://github.com/gre/shaderday.com/tree/master/blockarts/CryptoAliens).

It is implemented with [`gl-react`](https://github.com/gre/gl-react) which is convenient to write and compose [_GLSL Fragment Shaders_](https://www.khronos.org/opengl/wiki/Fragment_Shader).

> There are actually 3 shaders and a Canvas 2D texture at stake, here is the big picture of the pipeline:
> **TODO**

It also uses `mersenne-twister` library, a [PRNG](https://en.wikipedia.org/wiki/Pseudorandom_number_generator) used to get a wide and deterministic variety of shapes, initialized with the block hash. That said, as pointed in the previous section, the main features of the shape are determined by Ethereum block information itself (number of transactions, timestamp, transfers, gas used,...).

I kickstarted this whole idea on [Twitch](https://twitch.tv/greweb). A recording is [available on Youtube](https://www.youtube.com/watch?v=WUzOlLq0IAo). Apart from the many glitches this 3 hours session had remained to be solved, the main part of this was implemented that night. Indeed I had to work countlessly on polishing the shaders, lighting and work on post-processing.

The technique implemented on the main shader is [raymarching distance functions](https://www.iquilezles.org/www/articles/raymarchingdf/raymarchingdf.htm). The shapes at stake are mostly segments that are merged with a smooth union. There are many loops involved which made it challenging to optimize.
There may be issues on some mobile phone even tho it works on mine thanks to a "pixelated" version. (downscaling the pixels helped)

### How is Mandelglitch used?

As said, [Mandelglitch BlockStyle](https://ethblock.art/create/17) is re-used in this CryptoAliens BlockStyle. This really is the power of gl-react: it makes such composability really easy to do, the same way you can compose React components.

You can see in the [Youtube recording](https://www.youtube.com/watch?v=WUzOlLq0IAo) the way I have implemented it initially: it is just a simple import of Mandelglitch.js (literally the BlockStyle as-is) that I can just send as a uniform sampler2D.

```
<Node
  shader={sceneShaders.scene}
  uniforms={{
    t: <Mandelglitch block={block} mod1={mod1} mod2={mod2} mod3={mod3} />,
  ...
```

after that, it was simpler to embed Mandelglitch in the BlockStyle.

The way Mandelglitch texturing is used however is that I will only use the "red" component and remap it to CryptoAliens' own palette, in order to have a better control of the coloring.
