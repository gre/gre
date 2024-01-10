// Looking for a challenge? Copy this code in your computer! It's plottable!
import fs from "fs";
let alphabet = ",,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,4145+4647,2123+6163,4227+7257+1393+0585,7232232475867727+5158,,6131222387+84866727161524,5153,61434668,31535638,1476+7416+4347,1575+4347,465657394746,1585,3656573736,1871,26376776736232232663,135257+2787,124161731787,1151727354345475765717,8616156267,7222246475766727,7222131627778685743425,128237,547372613122234475867727162554,7465351412317182867727,4353544443+4656574746,4353544443+465657394746,831587,1484+1686,138517,2241617273544546+4748,,17425287+6535,116182836414648586671711,823214153787,,,822227+3484,,,6121+4147+2767,,,212787,07011145718187,,,,,,7222132475867717,,,,,,,,71414878,1178,21515828,,,3152,13637477+6525162767,2127+3363856737,8333153787,7177+6333153767,15856333153787,81514247+2383,73786929+6333153767,2127+2443637477,4151+235357+2787,4151+2353584919,2127+3573+5587,2151566787,1713+24334347+54637377,2327+3443637477,244363747667372624,2329+3363856737,7379+6333153767,3337+445383,7333243565766727,42465777+2373,2326375766+7773,13475783,0327374454677793,1387+1783,1347+833919,23732777,71515344141545565878,5159,21414354848555464828"
  .split(",").map(glyph => glyph.split("+").map(l =>
    Array(l.length / 2).fill(0).map((z, i) => [+l[i * 2], +l[i * 2 + 1]])));

let code = fs.readFileSync("index.mjs");
let width = 210;
let height = 297;
let pad = 10;
let fontsize = 4.2;
let fontratio = 31 / 60;
let yincr = 1.2 * fontsize;
let xincr = fontsize * fontratio;
let fontymul = fontsize / 10;
let fontxmul = fontratio * fontymul;
let paths = [];
let x = pad;
let y = pad;
for (let c of code) {
  let newline = c === 10;
  if (newline || x + xincr > width - pad) {
    x = pad;
    y += yincr;
    if (newline) continue;
  }
  for (let line of alphabet[c]) {
    paths.push(line.map(([x0, y0]) => [x + x0 * fontxmul, y + y0 * fontymul]));
  }
  x += xincr;
}

let layer = l => `<g inkscape:groupmode="layer" inkscape:label="${l.color}">${l.routes.map((route) => `<path d="${route
  .map(([x, y], i) => `${i === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`)
  .join(" ")}" fill="none" stroke="${l.color}" stroke-width="0.35" />`)
  .join("\n")}</g>`;

let svg = layers => `<svg viewBox="0 0 ${width} ${height}" width="${width}mm" height="${height}mm" xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape">${layers.map(layer).join("\n")}</svg>`;

fs.writeFileSync("0.svg", svg([{ color: "black", routes: paths }]));
