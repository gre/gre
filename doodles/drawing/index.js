import React, { Component, useState, useRef, useEffect } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";

const shaders = Shaders.create({
  colorize: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform vec4 color;
void main() {
  gl_FragColor = color;
}
  `,
  },
  paint: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform sampler2D backbuffer, brush;
uniform float painting, size, angle;
uniform vec2 pos;
uniform float color;
uniform float ratio;

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
    return a + b*cos( 6.28318*(c*t+d) );
}

void main() {
  // previous pixel color
  vec4 c = texture2D(backbuffer, uv);

  // reverse the position to lookup in the brush texture
  vec2 brushP = vec2(ratio, 1.0) * (uv - pos) / size;
  brushP *= mat2(
    cos(angle), sin(angle),
    -sin(angle), cos(angle)
  );
  brushP = (brushP + 1.0) / 2.0;

  // how much we want to draw of color
  float draw =
    painting *
    // this part ensure to not draw out of bound
    step(0.0, brushP.x) *
    step(0.0, brushP.y) *
    step(brushP.x, 1.0) *
    step(brushP.y, 1.0) *
    // look up the texture (intensity of brush at current pixel)
    (1.0 - texture2D(brush, brushP).r);

  vec3 clr = palette(color, vec3(0.5), vec3(0.5), vec3(1.0), vec3(0.3, 0.1, 0.9));

  // blend the brush color with the previous color based on draw
  c.rgb = mix(c.rgb, clr, draw);

  gl_FragColor = c;
}
  `,
  },
});

const Colorize = ({ color }) => (
  <Node shader={shaders.colorize} uniforms={{ color }} />
);

export class Paint extends Component {
  state = { initiated: false };
  onDraw = () => {
    if (!this.state.initiated) {
      this.setState({ initiated: true });
    }
  };
  shouldComponentUpdate(nextProps) {
    return nextProps.painting;
  }
  render() {
    const { color, size, pos, painting, angle, brush, ratio } = this.props;
    const backbuffer = this.state.initiated ? (
      Uniform.Backbuffer
    ) : (
      <Colorize color={[1, 1, 1, 1]} />
    );
    return (
      <Node
        shader={shaders.paint}
        onDraw={this.onDraw}
        backbuffering
        uniforms={{
          backbuffer,
          color,
          size,
          pos,
          brush,
          angle,
          painting,
          ratio,
        }}
      />
    );
  }
}

class Main extends Component {
  render() {
    const { color, size, drawing, center, brush, drawAngle, ratio } =
      this.props;

    // some brush will randomly rotate, other will follow touch move
    const angle = shouldRandomizeAngle(brush)
      ? 2 * Math.PI * Math.random()
      : drawAngle;

    return (
      <LinearCopy>
        <Paint
          color={color}
          size={size}
          pos={center}
          angle={angle}
          painting={drawing}
          brush={brush}
          ratio={ratio}
        />
      </LinearCopy>
    );
  }
}

const brushes = [
  require("./acrylic01.png"),
  require("./acrylic02.png"),
  require("./acrylic03.png"),
  require("./acrylic04.png"),
  require("./acrylic05.png"),
  /*
  require("./block01.png"),
  require("./block02.png"),
  require("./cell01.png"),
  require("./chalk03.png"),
  require("./hardness025.png"),
  require("./hardness050.png"),
  require("./hardness075.png"),
  require("./hardness100.png"),
  require("./oils01.png"),
  require("./pixel.png"),
  require("./star.png"),
  require("./texture01.png"),
  */
];

const whitelistRandomizeAngle = [
  require("./acrylic01.png"),
  require("./acrylic02.png"),
  require("./acrylic03.png"),
  require("./acrylic04.png"),
  require("./acrylic05.png"),
  require("./cell01.png"),
  require("./chalk03.png"),
  require("./oils01.png"),
  require("./texture01.png"),
];

function shouldRandomizeAngle(brush) {
  return whitelistRandomizeAngle.includes(brush);
}

const Example = () => {
  const { ref, width, height } = useDimensions({});

  const [drawing, setDrawing] = useState(false);
  const [color, setColor] = useState(0);
  const [center, setCenter] = useState([0.5, 0.5]);
  const [drawAngle, setDrawAngle] = useState(0);
  const [brush, setBrush] = useState(brushes[0]);
  const [size, setSize] = useState(0.1);

  const onMouseLeave = () => {
    setDrawing(false);
  };

  function syncColor() {
    setColor((Date.now() / 4000) % 1);
  }

  function getPosition(e) {
    const rect = e.target.getBoundingClientRect();
    let touch = e;
    if (e.touches) {
      touch = e.touches[0];
    }
    if (!touch) {
      return [0, 0];
    }
    return [
      (touch.clientX - rect.left) / rect.width,
      (rect.bottom - touch.clientY) / rect.height,
    ];
  }

  const onMouseMove = (e) => {
    if (drawing) {
      e.preventDefault();
      let next = getPosition(e);
      let x = !center ? 0 : next[0] - center[0];
      let y = !center ? 0 : next[1] - center[1];
      let nextAngle = x * x + y * y > 0.00001 ? Math.atan2(y, x) : drawAngle;
      setCenter(next);
      setDrawAngle(nextAngle);
      syncColor();
    }
  };

  const onMouseDown = (e) => {
    setBrush(brushes[Math.floor(Math.random() * brushes.length)]);
    setDrawing(true);
    setCenter(getPosition(e));
    setSize(0.05 + 0.2 * Math.random());
    syncColor();
  };

  const onMouseUp = () => {
    setDrawing(false);
  };

  const onTouchStart = (e) => onMouseDown(e);
  const onTouchEnd = (e) => onMouseUp(e);
  const onTouchCancel = (e) => onMouseUp(e);
  const onTouchMove = (e) => onMouseMove(e);

  const props = {
    color,
    size,
    drawing,
    center,
    brush,
    drawAngle,
    ratio: width / height,
  };

  return (
    <div ref={ref} style={{ width: "100vw", height: "100vh" }}>
      {width ? (
        <Surface
          width={width}
          height={height}
          onMouseLeave={onMouseLeave}
          onMouseMove={onMouseMove}
          onMouseDown={onMouseDown}
          onMouseUp={onMouseUp}
          onTouchStart={onTouchStart}
          onTouchEnd={onTouchEnd}
          onTouchCancel={onTouchCancel}
          onTouchMove={onTouchMove}
          style={{ cursor: "crosshair" }}
          preload={brushes}
        >
          <Main key={`${width}_${height}`} {...props} />
        </Surface>
      ) : null}
    </div>
  );
};

ReactDOM.render(<Example />, document.getElementById("root"));
