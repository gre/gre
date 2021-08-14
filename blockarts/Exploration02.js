import React from 'react';
import Sketch from 'react-p5';
import MersenneTwister from "mersenne-twister";

let DEFAULT_SIZE = 500;
const CustomStyle = ({
  block,
  canvasRef,
  attributesRef,
  width,
  height,
  handleResize,
  mod1 = 0.5,
  mod2 = 0.5,
  mod3 = 0.5,
  mod4 = 0.5,
}) => {
  const background = "black";
  const color1 = "red";

  const { hash } = block;
  const setup = (p5, canvasParentRef) => {
    let _p5 = p5.createCanvas(width, height).parent(canvasParentRef);
    canvasRef.current = p5;
    attributesRef.current = () => {
      return {
        attributes: [
          {
            trait_type: 'your trait here text',
            value: 'replace me',
          },
        ],
      };
    };
  };

  const draw = (p5) => {
    let dim = Math.min(width, height);

    p5.background(background);

    let seed = parseInt(hash.slice(0, 16), 16);
    const rng = new MersenneTwister(seed);

    let f = Math.floor(Math.sqrt(block.transactions.length));
    let objs = block.transactions.map((t, i) => {
      let input = t.input && t.input.slice(2) || ""
      if (input.length===0) return;
      const sz = Math.ceil(Math.sqrt(input.length / 8));
      return {
        x: (i % f) / f,
        y: Math.floor(i / f) / f,
        width: 4 * sz / dim,
        height: 4 * sz / dim,
        data: input,
        sz
      };
    }).filter(Boolean);

    p5.fill("red");
    p5.noStroke();

    objs.forEach((o) => {
      // TODO centered on diff ratio
      for (let xi = 0; xi < o.sz; xi++) {
        for (let yi = 0; yi < o.sz; yi++) {
          const x = dim * (o.x + o.width * xi / o.sz);
          const y = dim * (o.y + o.height * yi / o.sz);
          const i = 8 * (xi + yi * o.sz);
          p5.fill(p5.color(
            parseInt(o.data.slice(i, i+2)||0, 16),
            parseInt(o.data.slice(i+2, i+4)||0, 16),
            parseInt(o.data.slice(i+4, i+6)||0, 16)
          ));
          p5.rect(x, y, dim * o.width / o.sz, dim * o.height / o.sz)
          // p5.set(x, y, );
        }
      }
      // p5.updatePixels();
      /*
      p5.rect(
        dim * o.x,
        dim * o.y,
        dim * o.width,
        dim * o.height
      );
      */
    });
  };

  return <Sketch setup={setup} draw={draw} windowResized={handleResize} />;
};


export default CustomStyle