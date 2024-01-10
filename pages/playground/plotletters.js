import React, { useEffect, useState, useRef, useCallback, useMemo } from "react";

function Grid({ w, h }) {
  let fill = "#f00";
  return <g
    style={{ pointerEvents: "none" }}>
    {Array(11).fill(0).map((_, i) => {
      const x = i * w / 10;
      return <line key={i} x1={x} y1={0} x2={x} y2={h} stroke={fill} />
    })}
    {Array(11).fill(0).map((_, i) => {
      const y = i * h / 10;
      return <line key={i} x1={0} y1={y} x2={w} y2={y} stroke={fill} />
    })}

  </g>
}

function FontGuideline({ w, h, letter }) {
  let size = 0.8585 * h;
  return <text
    style={{ pointerEvents: "none" }} opacity={0.3} x={w / 2} y={0} textAnchor="middle" alignmentBaseline="before-edge" fontSize={size} fill="#f00" fontFamily="monospace">{letter}</text>
}

function Glyph({ value, w, h, strokeWidth }) {

  let last = value.length > 0 && value[value.length - 1];
  let cursor = last && last.length > 0 && last[last.length - 1];


  return <g style={{ pointerEvents: "none" }}>

    {value.map((line, i) => {
      return <path key={i}
        opacity={0.8}
        strokeLinecap="round"
        strokeLinejoin="round"
        d={
          line.map((p, i) => (i === 0 ? "M" : "L") + (w * (p[0] + 0.5) / 10) + "," + (h * (p[1] + 0.5) / 10)).join(" ")} stroke="#000" fill="none" strokeWidth={strokeWidth} />
    })}
    {cursor && <circle cx={w * (cursor[0] + 0.5) / 10} cy={h * (cursor[1] + 0.5) / 10} r={0.3 * strokeWidth} fill="#f00" />}
  </g>
}

function Editor({
  letter,
  // Array of lines
  // line: Array of points.
  // point: [x, y], where x is in 0..9, y is in 0..9
  value,
  setValue,
}) {
  let w = 310;
  let h = 600;


  useEffect(() => {
    if (typeof document !== "undefined") {
      let handler = event => {
        event.preventDefault();
        let last = value.length > 0 && value[value.length - 1];
        if (last && last.length == 0) {
          setValue([...value].slice(0, -2).concat([[]]));
        }
        else {
          setValue([...value, []]);
        }
      }
      document.addEventListener('contextmenu', handler);
      return () => {
        document.removeEventListener('contextmenu', handler);
      }
    }
  }, [value]);


  const onClick = useCallback((e) => {
    // right click
    if (e.button === 2) {
      return;
    }

    const rect = e.target.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const x0 = Math.max(0, Math.min(Math.floor(x / w * 10), 9));
    const y0 = Math.max(0, Math.min(Math.floor(y / h * 10), 9));
    const append = [...value];
    if (append.length === 0) {
      append.push([]);
    }
    const last = append[append.length - 1];
    last.push([x0, y0]);
    setValue(append);
  }, [value]);

  return <svg style={{
    overflow: "visible",
    cursor: "crosshair"
  }} width={w} height={h} onClick={onClick}>
    <Grid w={w} h={h} />
    <FontGuideline w={w} h={h} letter={letter} />
    <Glyph w={w} h={h} strokeWidth={0.1 * w} value={value} />
  </svg>
}

class Glyphs {
  letters = {};

  constructor() {
    if (typeof localStorage === "undefined") {
      return;
    }
    const db = JSON.parse(localStorage.getItem("glyphs") || "{}");
    const letters = {};
    for (const letter in db) {
      const v = db[letter];
      letters[letter] = v.split("+").map(line => {
        let split = line.split("");
        const points = [];
        while (split.length) {
          const x = parseInt(split.shift(), 10);
          const y = parseInt(split.shift(), 10);
          points.push([x, y]);
        }
        return points;
      });
    }
    this.letters = letters;
  }

  setLetter(letter, glyph) {
    this.letters[letter] = glyph;
    if (glyph.length === 0) {
      delete this.letters[letter];
    }
    this.save();
  }

  getLetter(letter) {
    return this.letters[letter];
  }

  getEncoded() {
    const array = Array(255).fill("");
    let max = -1;
    for (const letter in this.letters) {
      let ascii = letter.charCodeAt(0);
      max = Math.max(max, ascii);
      array[ascii] = this._encodeGlyph(this.letters[letter]);
    }
    array.length = max + 1;
    return array.join(",");
  }

  save() {
    const obj = {};
    for (const letter in this.letters) {
      let v = this.letters[letter];
      obj[letter] = this._encodeGlyph(v);
    }
    localStorage.setItem("glyphs", JSON.stringify(obj));
  }

  _encodeGlyph(glyph) {
    return glyph.filter(line => line.length > 1)
      .map(line => line
        .map(p => p[0].toFixed(0) + p[1].toFixed(0))
        .join(""))
      .join("+")
  }
}

export default function Main() {
  const [letter, setLetter] = useState("");
  let db = useMemo(() => new Glyphs(), []);
  const [value, setValue] = useState([]);
  const onChangeLetter = useCallback((e) => {
    setLetter(e.target.value);
    const value = db.getLetter(e.target.value) || [];
    setValue(value);
  }, []);

  const onSetValue = useCallback((value) => {
    setValue(value);
    db.setLetter(letter, value);
  }, [letter]);

  const onReset = useCallback(() => {
    onSetValue([]);
  }, [onSetValue]);

  const onNewLine = useCallback(() => {
    onSetValue([...value, []]);
  }, [onSetValue, value]);

  const exportToClipboard = useCallback(() => {
    navigator.clipboard.writeText(db.getEncoded());
  }, []);

  return (
    <div>
      <div style={{ padding: 10 }}>
        <input style={{ width: 80, fontFamily: "monospace" }} type="text" placeholder="letter" value={letter} onChange={onChangeLetter} />
        <button onClick={onReset}>Reset</button>
        <button onClick={onNewLine}>New Line</button>
        <button onClick={exportToClipboard}>Export</button>
      </div>
      <div style={{ display: "flex", flexDirection: "row", }}>
        <Editor letter={letter} value={value} setValue={onSetValue} />
        <div style={{ padding: 20, fontFamily: "monospace", fontSize: 24 }}>
          {Object.keys(db.letters).map((letter, i) =>
            <span key={i} style={{ display: "inline-block", border: "1px solid black", margin: 4, fontFamily: "monospace", cursor: "pointer" }} onClick={() => {
              setLetter(letter);
              setValue(db.getLetter(letter));
            }}>{letter}</span>
          )}
        </div>
      </div>
    </div>
  );
}
