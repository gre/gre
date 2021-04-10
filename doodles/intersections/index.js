import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";

function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}

const shaders = Shaders.create({
  scene: {
    frag: GLSL`
#version 300 es
precision highp float;
in vec2 uv;
out vec4 color;
uniform vec2 resolution;
uniform float time;
#define PI ${Math.PI}

float sdCircle(vec2 p, float r) {
  return length(p) - r;
}
float sdBox( in vec2 p, in vec2 b ) {
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a))*r;
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}
vec2 pModMirror2(inout vec2 p, vec2 size) {
	vec2 halfsize = size*0.5;
	vec2 c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	p *= mod(c,vec2(2.))*2. - vec2(1.);
	return c;
}

float shape (vec2 p, float d) {
  float t = time + 0.01 * d;
  p -= 0.5;
  p *= 1.0 + 0.02 * min(pow(1.+t, 0.9), 400.);
  p.y += 0.001 * sin(t) * d;
  pModMirror2(p, vec2(0.4 + 1. / t));
  float size = 0.5 + abs(0.5 * cos(0.2 * t)); 
  float s = sdCircle(p, 0.4 * size);
  s = max(s, sdBox(p + vec2(0.4 * size * cos(2. * t), 0.), vec2(0.2 * size)));
  pR(p, -0.5 * t);
  float a = pModPolar(p, 8.0);
  p.x -= 0.3 * size + smoothstep(10., 30., t) * 0.05 * cos(PI * a + 4. * t);
  float boxes = sdBox(p, vec2(0.05 * pow(1.+t, 0.1) * size));
  s = max(min(s, boxes), -max(s, boxes));
  return smoothstep(0.0, 0.0005, s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;  
  color = vec4(
    shape(base, -1.),
    shape(base, 1.),
    shape(base, 0.5),
    1.0);
}
    `,
  },
});

const Scene = () => {
  const time = useTime();
  return (
    <Node
      shader={shaders.scene}
      uniforms={{
        resolution: Uniform.Resolution,
        time,
      }}
    />
  );
};

const Main = () => {
  const { ref, width, height } = useDimensions({});
  return (
    <div ref={ref} style={{ width: "100vw", height: "100vh" }}>
      <Surface width={width} height={height}>
        <Scene />
      </Surface>
    </div>
  );
};

ReactDOM.render(<Main />, document.getElementById("main"));
