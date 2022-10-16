const width = parseInt(process.argv[2]);
const height = parseInt(process.argv[3]);
const grid = process.argv[4].split("x").map((o) => parseInt(o));

const expectedlength = 5 + (grid[0] + 1) * (grid[1] + 1);
if (process.argv.length !== expectedlength) {
  console.error(
    "expected " + expectedlength + " params and not " + process.argv.length
  );
  process.exit(1);
}

let circular = process.env.LOOPMODE === "circle";
let isvertical = (process.env.ORIENTATION || "horizontal") === "vertical";

let ymul = grid[0] + 1;
let xmul = 1;
function iter(xi, y) {
  let x = circular && y % 2 == 1 ? grid[0] - 1 - xi : xi;
  const topleft = process.argv[5 + xmul * x + ymul * y];
  const topright = process.argv[5 + xmul * (x + 1) + ymul * y];
  const bottomleft = process.argv[5 + xmul * x + ymul * (y + 1)];
  const bottomright = process.argv[5 + xmul * (x + 1) + ymul * (y + 1)];
  console.log(
    `${topleft} 0,0 ${bottomleft} 0,${height} ${bottomright} ${width},${height} ${topright} ${width},0`
  );
}

if (isvertical) {
  for (let xi = 0; xi < grid[0]; xi++) {
    for (let y = 0; y < grid[1]; y++) {
      iter(xi, y);
    }
  }
} else {
  for (let y = 0; y < grid[1]; y++) {
    for (let xi = 0; xi < grid[0]; xi++) {
      iter(xi, y);
    }
  }
}
