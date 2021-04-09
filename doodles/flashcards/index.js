import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { TravellingSalesmanSolver } from "./tsp";
import hsk3 from "./HSK3.js";

const lines = hsk3.split("\n").filter(Boolean);

const daysSince2010 = Math.floor(
  (Date.now() - new Date("2020-01-01")) / (24 * 60 * 60 * 1000)
);
const dayLine = lines[daysSince2010 % lines.length];
const cells = dayLine.split("\t");
const dayChar = cells[1];
const pinyin = cells[3];
const translation = cells[4];

const dim = 400;
const strokeWidth = 2;
const canvas = document.createElement("canvas");
canvas.width = dim;
canvas.height = dim;
console.log(canvas.width, canvas.height);
const ctx = canvas.getContext("2d");
ctx.fillStyle = "#000";
ctx.fillRect(0, 0, canvas.width, canvas.height);
ctx.textAlign = "center";
ctx.textBaseline = "middle";
ctx.font = Math.floor((1.4 * dim) / (dayChar.length + 1)) + "px Arial";
ctx.fillStyle = "#fff";
ctx.fillText(dayChar, dim / 2, 0.5 * dim);

const imageData = ctx.getImageData(0, 0, dim, dim);

function sampling(size) {
  const candidates = [];
  for (let y = 0; y < dim; y++) {
    for (let x = 0; x < dim; x++) {
      const i = 4 * (y * dim + x);
      const inside = imageData.data[i] > 127;
      if (inside) {
        candidates.push([x, y]);
      }
    }
  }
  candidates.sort(() => Math.random() - 0.5);
  candidates.splice(size);
  return candidates;
}

let lastUpdate = 0;
const Main = () => {
  const [data] = useState(() => ({
    points: sampling(2000),
    rev_x: false,
    rev_y: false,
    scale: 1,
  }));
  const [solver, setSolver] = useState(null);
  const [route, setRoute] = useState(() => []);

  useEffect(() => {
    let finished = false;
    const solver = new TravellingSalesmanSolver(data, (path) => {
      let t = Date.now();
      if (t - lastUpdate > 2000) {
        setRoute(path);
        lastUpdate = t;
      }
      return new Promise((done) => setTimeout(() => done(!finished), 0));
    });
    setSolver(solver);

    return () => {
      finished = true;
    };
  }, [data]);

  const skipThreshold = 0.03 * dim;

  let last = [-100, -100];
  const pathData = !solver
    ? null
    : route
        .map((index) => {
          const node = solver.nodes[index];
          const { abs_x, abs_y } = node;
          const dx = abs_x - last[0];
          const dy = abs_y - last[1];
          last = [abs_x, abs_y];
          const skip = Math.sqrt(dx * dx + dy * dy) > skipThreshold;
          return `${skip ? "M" : "L"}${abs_x},${abs_y}`;
        })
        .join(" ") + " z";

  return (
    <>
      <svg width="100%" height="100%" viewBox={`0 0 ${dim} ${dim}`}>
        <path
          transform="translate(0.5, 0.5)"
          d={pathData}
          stroke="black"
          strokeWidth={strokeWidth}
          fill="none"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
        <path
          d={pathData}
          stroke="white"
          strokeWidth={strokeWidth}
          fill="none"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
      <div id="pinyin">{pinyin}</div>
      <div id="translation">{translation}</div>
    </>
  );
};

ReactDOM.render(<Main />, document.getElementById("main"));
