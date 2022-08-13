import perlin from "./perlin.mjs";
//-----+++++++++++++-----
//##### ASCII FIELD #####
// a plottable generator
//        by @greweb 2022
//-----------------------
// entry for @sableraph wccchallenge 'ASCII'

const WIDTH = 210;
const HEIGHT = 297;
const PAD = 10;
const SIZE = 3; // size of one cell in millimeters

//#################################################
//          _(,__           __),
//      (_,d888888888b,d888888888b
//       d888888888888/888888888888b_)
//    (_8888888P'""'`Y8Y`'""'"Y88888b
//      Y8888P.-'     `      '-.Y8888b_)
//     ,_Y88P (_(_(        )_)_) d88Y_,
//      Y88b,  (o  )      (o  ) d8888P
//      `Y888   '-'        '-'  `88Y`
//      ,d/O\         c         /O\b,
// jgs    \_/'.,______w______,.'\_/
//
//----++++++----
//+++ SHAPES +++
// represented on a 0..1 square coordinates space
const shapes = [
  [
    // -
    [
      [0.1, 0.5],
      [0.9, 0.5],
    ],
  ],
  [
    // +
    [
      [0.1, 0.5],
      [0.9, 0.5],
    ],
    [
      [0.5, 0.1],
      [0.5, 0.9],
    ],
  ],
  [
    // o
    Array(12)
      .fill(null)
      .map((_, i) => {
        const a = (Math.PI * 2 * i) / 11;
        return [0.5 + 0.25 * Math.cos(a), 0.5 + 0.3 * Math.sin(a)];
      }),
  ],
  [
    // #
    [
      [0.1, 0.4],
      [0.9, 0.4],
    ],
    [
      [0.1, 0.6],
      [0.9, 0.6],
    ],
    [
      [0.4, 0.1],
      [0.4, 0.9],
    ],
    [
      [0.6, 0.1],
      [0.6, 0.9],
    ],
  ],
];

//#################################################
//
// |  _________________  |
// | |              /  | |
// | |       /\    /   | |
// | |  /\  /  \  /    | |
// | | /  \/    \/     | |
// | |/             JO | |
// | |_________________| |
// |  __ __ __ __ __ __  |
// | |__|__|__|__|__|__| |
// | |__|__|__|__|__|__| |
// | |__|__|__|__|__|__| |
// | |__|__|__|__|__|__| |
// | |__|__|__|__|__|__| |
// | |__|__|__|__|__|__| |
// |  ___ ___ ___   ___  |
// | | 7 | 8 | 9 | | + | |
// | |___|___|___| |___| |
// | | 4 | 5 | 6 | | - | |
// | |___|___|___| |___| |
// | | 1 | 2 | 3 | | x | |
// | |___|___|___| |___| |
// | | . | 0 | = | | / | |
// | |___|___|___| |___| |
// |_____________________|
//---+++++++---
//+++ VALUE +++
// a function that tell the character of (x,y)
//--------------
function mkvalue(rand) {
  let seed = rand(9999);
  // noise frequencies control the zoom in the noise
  let f1 = 0.01 + rand(0.02);
  let f2 = f1 * (1 + rand(2) * rand()) + (rand() < 0.2 ? rand() : 0);
  // random amplitudes
  let amp1 = 1 + rand(2) * rand(); // this control the spread of the white area
  let amp2 = 2 + rand(2); // this control the effect of the domain warping effect
  // offset the noise to create white areas
  let offset = rand(0.4) - 0.2;

  return function value(x, y) {
    x = Math.abs(x - WIDTH / 2); // x-mirror
    return (
      offset +
      amp1 *
        perlin.perlin3(
          seed +
            amp2 *
              perlin.perlin3(
                // including a second level of perlin noise inside perlin noise
                // is a technique called "domain warping"
                seed * 3.3 - 7.7,
                f2 * x,
                f2 * y
              ),
          f1 * x,
          f1 * y
        )
    );
  };
}

//             _
//            H||
//            H||
//  __________H||___________
// [|.......................|
// ||.........## --.#.......|
// ||.........   #  # ......|            @@@@
// ||.........     *  ......|          @@@@@@@
// ||........     -^........|   ,      - @@@@
// ||.....##\        .......|   |     '_ @@@
// ||....#####     /###.....|   |     __\@ \@
// ||....########\ \((#.....|  _\\  (/ ) @\_/)____
// ||..####,   ))/ ##.......|   |(__/ /     /|% #/
// ||..#####      '####.....|    \___/ ----/_|-*/
// ||..#####\____/#####.....|       ,:   '(
// ||...######..######......|       |:     \
// ||.....""""  """"...b'ger|       |:      )
// [|_______________________|       |:      |
//        H||_______H||             |_____,_|
//        H||________\|              |   / (
//        H||       H||              |  /\  )
//        H||       H||              (  \| /
//       _H||_______H||__            |  /'=.
//     H|________________|           '=>/  \
//                                  /  \ /|/
//                                ,___/|             (ascii by Joris Bellenger)
//
//----+++++-----
//++++ ART +++++
// the main function
//--------------
function art(S) {
  // RNG
  let t, s;
  let rand = (a = 1) =>
    a *
    ((t = S[3]),
    (S[3] = S[2]),
    (S[2] = S[1]),
    (s = S[1] = S[0]),
    (t ^= t << 11),
    (S[0] ^= t ^ (t >>> 8) ^ (s >>> 19)),
    S[0] / 2 ** 32);

  let routes = [];

  let value = mkvalue(rand);

  for (let x = PAD; x < WIDTH - PAD - SIZE; x += SIZE) {
    for (let y = PAD; y < HEIGHT - PAD - SIZE; y += SIZE) {
      let n = value(x, y);
      let index = ((n + 0.5) * shapes.length) | 0;
      (shapes[index] || []).forEach((route) => {
        routes.push(route.map(([px, py]) => [x + SIZE * px, y + SIZE * py]));
      });
    }
  }

  return { routes };
}

//#################################################
//          _______
//         |.-----.|
//         ||x . x||
//         ||_.-._||
//         `--)-(--`
//        __[=== o]___
//       |:::::::::::|\
// jgs   `-=========-`()
//
// Finally we make a helper that will bake the SVG.
function makeSVG(a) {
  let layer = a.routes
    .map(
      (route) =>
        `<path d="${route
          .map(
            ([x, y], i) =>
              `${i === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`
          )
          .join(
            " "
          )}" fill="none" stroke="black" stroke-width="0.35" style="mix-blend-mode: multiply;" />`
    )
    .join("\n");
  return `<svg style="background:white" viewBox="0 0 210 297" width="210mm" height="297mm" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">
<g inkscape:groupmode="layer" inkscape:label="black">${layer}</g>
</svg>`;
}

const a = art(
  Uint32Array.from([0, 0, 0, 0].map(() => (Math.random() * 0xffffffff) | 0))
);
const svg = makeSVG(a);
document.body.innerHTML = svg;

console.log(svg);
