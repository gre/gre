const width = process.argv[2].split("x").map((o) => parseInt(o));
const height = process.argv[3].split("x").map((o) => parseInt(o));
const grid = process.argv[4].split("x").map((o) => parseInt(o));

const expectedlength = 5 + (grid[0] + 1) * (grid[1] + 1);
if (process.argv.length !== expectedlength) {
  console.error(
    "expected " + expectedlength + " params and not " + process.argv.length
  );
  process.exit(1);
}

for (let y = 0; y < grid[1]; y++) {
  for (let x = 0; x < grid[0]; x++) {
    const topleft = process.argv[5 + x + (grid[0] + 1) * y];
    const topright = process.argv[5 + (x + 1) + (grid[0] + 1) * y];
    const bottomleft = process.argv[5 + x + (grid[0] + 1) * (y + 1)];
    const bottomright = process.argv[5 + (x + 1) + (grid[0] + 1) * (y + 1)];
    console.log(
      `${topleft} 0,0 ${bottomleft} 0,${height} ${bottomright} ${width},${height} ${topright} ${width},0`
    );
  }
}
