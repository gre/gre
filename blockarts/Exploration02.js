import React, { useRef } from 'react';
import Sketch from 'react-p5';
import MersenneTwister from "mersenne-twister";

export const styleMetadata = {
  name: "",
  description: "",
  image: "",
  creator_name: "greweb",
  options: {
    // comment seed when going production!
    seed: 0, // this was used for debug
    mod1: 0.5,
    mod2: 0.5,
    mod3: 0.1,
  },
};

let DEFAULT_SIZE = 500;
const CustomStyle = ({
  block,
  canvasRef,
  attributesRef,
  width,
  height,
  handleResize,
  mod1 = 0.75, // Example: replace any number in the code with mod1, mod2, or color values
  mod2 = 0.25,
  color1 = '#4f83f1',
  background = '#ccc',
}) => {
  const shuffleBag = useRef();
  const hoistedValue = useRef();

  const { hash } = block;

  // setup() initializes p5 and the canvas element, can be mostly ignored in our case (check draw())
  const setup = (p5, canvasParentRef) => {
    // Keep reference of canvas element for snapshots
    let _p5 = p5.createCanvas(width, height).parent(canvasParentRef);
    canvasRef.current = p5;

    attributesRef.current = () => {
      return {
        // This is called when the final image is generated, when creator opens the Mint NFT modal.
        // should return an object structured following opensea/enjin metadata spec for attributes/properties
        // https://docs.opensea.io/docs/metadata-standards
        // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1155.md#erc-1155-metadata-uri-json-schema

        attributes: [
          {
            display_type: 'number',
            trait_type: 'your trait here number',
            value: hoistedValue.current, // using the hoisted value from within the draw() method, stored in the ref.
          },

          {
            trait_type: 'your trait here text',
            value: 'replace me',
          },
        ],
      };
    };
  };

  // draw() is called right after setup and in a loop
  // disabling the loop prevents controls from working correctly
  // code must be deterministic so every loop instance results in the same output

  // Basic example of a drawing something using:
  // a) the block hash as initial seed (shuffleBag)
  // b) individual transactions in a block (seed)
  // c) custom parameters creators can customize (mod1, color1)
  // d) final drawing reacting to screen resizing (M)
  const draw = (p5) => {
    let WIDTH = width;
    let HEIGHT = height;
    let DIM = Math.min(WIDTH, HEIGHT);
    let M = DIM / DEFAULT_SIZE;

    p5.background(background);

    // reset shuffle bag
    let seed = parseInt(hash.slice(0, 16), 16);
    shuffleBag.current = new MersenneTwister(seed);
    let objs = block.transactions.map((t) => {
      let seed = parseInt(t.hash.slice(0, 16), 16);
      return {
        y: shuffleBag.current.random(),
        x: shuffleBag.current.random(),
        radius: seed / 1000000000000000,
      };
    });

    // example assignment of hoisted value to be used as NFT attribute later
    hoistedValue.current = 42;

    objs.map((dot, i) => {
      p5.stroke(color1);
      p5.strokeWeight(1 + mod2 * 10);
      p5.ellipse(
        200 * dot.y * 6 * M,
        100 * dot.x * 6 * M,
        dot.radius * M * mod1
      );
    });
  };

  return <Sketch setup={setup} draw={draw} windowResized={handleResize} />;
};


export default CustomStyle