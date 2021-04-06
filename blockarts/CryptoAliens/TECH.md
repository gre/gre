# CryptoAliens: Genesis, a technical look

This article explains how [CryptoAliens: Genesis (ethblock.art)](README.md) works in technical depth.

<img src="previews/032.png" width="50%" /><img src="previews/036.png" width="50%" />

First of all, I would like to point out the [source code is available here on Github](https://github.com/gre/shaderday.com/tree/master/blockarts/CryptoAliens).

This whole idea was kicked off on [Twitch](https://twitch.tv/greweb). A recording is [available on Youtube](https://www.youtube.com/watch?v=WUzOlLq0IAo). Apart from the many glitches this 3 hours session had remained to be solved, the main part of this was implemented that night. Indeed I had to work countlessly on polishing the shaders, lighting and post-processing. I also spent a lot of time using the block data in a meaningful way because it's what [EthBlock.art](https://ethblock.art) really is about.

## EthBlock.art revolutionary idea

Before going further into the technical details of CryptoAliens, I would like to point out how revolutionary this EthBlock.art idea is.

The project aims to create a virtuous ecosystem of "deterministic art", code visualization of Ethereum blocks. Everything is data: from the ethereum block of transactions, to the code that visualize it, and to the NFTs minted/traded using Ethereum transactions (that themselves are into Ethereum blocks).

**This is a virtuous ecosystem, similar to [Supply Chain Transformation concepts](https://en.wikipedia.org/wiki/Value_chain): each actor in this ecosystem add value and get retributed for it, as I tried to explain in this schema:**

![](previews/ethblockart.png)

`CryptoAliens: Genesis` is one possible BlockStyle that I've designed, as a creative coder. It tries to visualize what happened in the Ethereum Block and will take mods into account to try to be as good as possible to deliver interesting possibilities to BlockArt minters.

## Ok, so how is it implemented technically?

Indeed WebGL.

More precisely, it is implemented with [`gl-react`](https://github.com/gre/gl-react) which is convenient to write and compose [_GLSL Fragment Shaders_](https://www.khronos.org/opengl/wiki/Fragment_Shader).

**here is the big picture of the pipeline:**

![](previews/graph.gif)

There are 2 main shaders: Mandelglitch (for skin texturing) and Scene (the main raymarching shader). Each of them take a bunch of parameters. `mod1..4` are values from the creator. The rest are inferred from the Block information, they are split into multiple parameters for convenience.

The parameters `s1..9` are coming directly from `mersenne-twister` library, a [PRNG](https://en.wikipedia.org/wiki/Pseudorandom_number_generator) used to get a wide and deterministic variety of shapes, initialized with the block hash. That said, as pointed in the previous section, the main features of the shape are determined by Ethereum block information itself (number of transactions, timestamp, transfers, gas used,...).

On top of these, some other parameters are controling key elements they come from more block information (heavy, head, bonesK, arms info,...).

The technique implemented on the main Scene shader is [raymarching distance functions](https://www.iquilezles.org/www/articles/raymarchingdf/raymarchingdf.htm). The shapes at stake are mostly segments that are merged with a smooth union. There are many loops involved which made it challenging to optimize.
There may be issues on some mobile phone even tho it works on mine thanks to a "pixelated" version. (downscaling the pixels helped)

Because of an heavy usage of this technique, the main scene was really challenging to optimize, it actually runs something like 5 FPS. My choice was to not animate it directly (you can see on the rendering graph that actually the buffer don't need to refresh, except when a "mod" is changed). However, thanks to intermediary framebuffers we can do 60 FPS animation at the postprocessing level. This is what the final user will get as the mods are static.

### Canvas 2D to draw texts

A Canvas 2D element is used to draw the text that will appear on top of everything.

```js
/// canvas used to draw in postprocessing ///
function FrameText({ blockNumber, dateText, width, height, kg, bones }) {
  const onCanvasRef = (canvas) => {
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const w = ctx.canvas.width;
    const h = ctx.canvas.height;
    const pad = Math.round(w * 0.01);
    const padX = Math.round(w * 0.02);
    const fontS = Math.floor(0.19 * w) / 10;
    const fontSize = `${fontS}px`;
    ctx.save();
    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, w, h);
    ctx.font = "bold " + fontSize + " monospace";
    ctx.textBaseline = "top";
    ctx.fillStyle = "#fff";
    ctx.fillText(`CryptoAliens specimen #${blockNumber}`, padX, pad);
    ctx.textBaseline = "bottom";
    ctx.textAlign = "right";
    ctx.font = fontSize + " monospace";
    ctx.fillText(
      `born ${dateText}, ${kg} kg, ${bones} bones`,
      w - padX,
      h - pad
    );
    ctx.restore();
  };
  return (
    <canvas
      ref={onCanvasRef}
      width={String(width * 2)}
      height={String(height * 2)}
    />
  );
}
const FrameTextCached = React.memo(FrameText);
```

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

### Code organisation

React and Gl-React allows to organize the code relatively easily. First of all each pass in the rendering scene is a component, then shaders are organized in the `Shaders.create` usage. I've tried to collocate them (still in same one big file to simplify the upload to EthBlock.art).

I find it pretty convenient to externalize piece of the logic into "hooks" function. Example:

```js
const CustomStyle = (props) => {
  // prettier-ignore
  const { block, attributesRef, mod1, mod2, mod3, mod4, highQuality, width, height } = props;
  // prettier-ignore
  const { kg, bones, theme, background, s1, s2, s3, s4, s5, s6, s7, s8, heavy, head, bonesK, armsLen, armsSpread, armsCenter, armsEndW, dateText, blockNumber } =
    useBlockDerivedData(block, mod1, mod2, mod3, mod4);

  useAttributesSync(attributesRef, kg, bones, theme);

  return (
    <LiveTV
      text={
        <FrameTextCached ... />
      }
      ...
    >
      <NearestCopy width={w} height={h}>
        <Scene
          t={<MandelglitchCached ... />}
          ...
        />
      </NearestCopy>
    </LiveTV>
  );
};
```

`useBlockDerivedData` internally uses `useMemo` in order to cache the computation of block data interpretation.

In order to make **only** one part of the tree to actively re-render, i've used a local `useTime` that would re-render only that part (the LiveTV final shader). It's implementation is trivial:

```js
function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}
```

---

My name is Gaëtan Renaudeau, and I'm a noise explorer. **feel free to ping me on Twitter [@greweb](https://twitter.com/greweb)**
