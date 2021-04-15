import React, { useState, useMemo, useEffect } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform, LinearCopy } from "gl-react";
import { Surface } from "gl-react-dom";
import useDimensions from "react-cool-dimensions";
import { useSpring, animated } from "react-spring";
import MersenneTwister from "mersenne-twister";

export function useTime() {
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

const Main = ({ time, values: [s1, s2, s3, s4] }) => (
  <Node
    shader={shaders.main}
    uniforms={{
      resolution: Uniform.Resolution,
      time,
      s1,
      s2,
      s3,
      s4,
    }}
  />
);

const AnimatedMain = animated(Main);

export const Scene = ({ n, time }) => {
  const valuesMemo = useMemo(() => {
    const rng = new MersenneTwister(n);
    return Array(4)
      .fill(null)
      .map(() => rng.random());
  }, [n]);
  const { values } = useSpring({
    values: valuesMemo,
    config: {
      mass: 1,
      tension: 50,
      friction: 20,
    },
  });
  return (
    <LinearCopy>
      <AnimatedMain time={time} values={values} />
    </LinearCopy>
  );
};

const Root = () => {
  const time = useTime();
  const [n, setN] = useState(() => Date.now());
  const { ref, width, height } = useDimensions({});
  function onClick() {
    setN((n) => n + 1);
  }
  return (
    <div
      onClick={onClick}
      ref={ref}
      style={{ cursor: "pointer", width: "100vw", height: "100vh" }}
    >
      <Surface width={width} height={height}>
        <Scene n={n} time={time} />
      </Surface>
    </div>
  );
};

const shaders = Shaders.create({
  main: {
    frag: GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;

uniform float time;
uniform float s1,s2,s3,s4;

#define PI ${Math.PI}

#define HIT vec4
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
void pR(inout vec2 p, float a) {
  p = cos(a) * p + sin(a) * vec2(p.y, -p.x);
}

float sdRoundBox( vec3 p, vec3 b, float r ) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0) - r;
}

vec3 normal (in vec3 p) {
  vec3 eps = vec3(0.001, 0.0, 0.0);
  return normalize(vec3(
    map(p+eps.xyy).x-map(p-eps.xyy).x,
    map(p+eps.yxy).x-map(p-eps.yxy).x,
    map(p+eps.yyx).x-map(p-eps.yyx).x
  ));
}

float diffuse(vec3 p, vec3 n, vec3 lpos) {
  vec3 l = normalize(lpos-p);
  float dif = clamp(dot(n, l), 0.01, 1.);
  return dif;
}

HIT marcher (inout vec3 p, vec3 dir) {
  // raymarching perf technique from https://www.shadertoy.com/view/XsyGWV
  HIT hit = HIT(0.);
  float precis = 0.0001;
  float t = 0.;
  for (int i=0; i<120; i++) {
    HIT h = map(p + t * dir);
    t += h.x;
    if (abs(h.x) < precis || p.z > 20.) {
      hit = h;
      break;
    }
  }
  p += t * dir;
  return hit;
}

HIT opU (HIT a, HIT b) {
  if (a.x < b.x) return a;
  return b;
}

float specularStrength (float m) {
  if (m<1.) return .0;
  return 0.2;
}

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(-1., 1., -5.);
  ldir = normalize(l - p);
  c +=
  vec3(2.) * (
    // ambient
    0.05
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
    + specular(n, hit.y, ldir, dir, 80.)
  );
  return c;
}

vec3 shade (HIT hit, vec3 g) {
  float m = hit.y;
  if (m < 1.) {
    return vec3(1.);
  }
  vec2 p = hit.zw;
  return vec3(
    smoothstep(0.16, 0.18, fract(-2. * time + 80. * s4 * (s1+0.5) * p.x + 60. * s4 * s3 *  p.y))
  );
}

HIT obj (vec3 p) {
  p.y -= 1.;
  pR(p.xz, 0.1 * time);
  pR(p.xy, 3. + 3. * (s1-0.5));
  pR(p.yz, 3. * (s3-0.5));
  p.x += 0.2 * s1 * s4 * cos(30. * s1 * s1 * p.z);
  p.y += 0.1 * s1 * (1. - s4) * sin(20. * s2 * s2 * p.x);
  p.z += 0.1 * s1 * s3 * cos(10. * s3 * s3 * p.y);
  vec2 xy = p.zy;

  float s = sdRoundBox(p, vec3(0.1 + 0.8 * (1. - s3)), 0.7 * s3);
  return HIT(s, 2.0, xy);
}

HIT map (vec3 p) {
  HIT s = HIT(min(20. - length(p),p.y), 0.1, 0., 0.);
  return opU(s, obj(p));
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}

vec3 scene(vec2 uvP) {
  float amp = 4.;
  float a = 0.;
  vec3 origin = vec3(amp * cos(a), 2., amp * sin(a));
  vec3 poi = vec3(0.0, 1., 0.0);
  vec3 c = vec3(0.);
  vec3 dir = normalize(vec3(uvP - .5, 1.5));
  dir = lookAt(origin, poi) * dir;
  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  // mist
  c = mix(c, vec3(1.), smoothstep(8.0, 16.0, length(origin - p)));
  return c;
}

void main() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;
  c = scene(base);
  
  /*
    for (float x=-.5; x<=.5; x += 1.) {
      for (float y=-.5; y<=.5; y += 1.) {
        vec2 d = 0.5 * vec2(x,y) / resolution;
        vec2 p = base + d;
        c += scene(p);
      }
    }
    c /= 5.0;
    */
  
  gl_FragColor = vec4(c, 1.0);
}
  `,
  },
});

ReactDOM.render(<Root />, document.getElementById("main"));
