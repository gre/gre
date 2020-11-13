const NORM = Math.sqrt(2) / 2;

export default (p, factor, total) => {
  const f = factor * 2 * Math.ceil(p / 2) / total;
  switch ((p - 1) % 4) { // alternate horizontal, vertical and 2 diagonals
    case 0:
      return [f, 0];
    case 1:
      return [0, f];
    case 2:
      return [f * NORM, f * NORM];
    case 3:
      return [f * NORM, -f * NORM];
  }
};
