// @flow
/**
 * LICENSE CC BY-NC-ND 4.0
 * Author: greweb – 2022 – New Wave
 */
import React, { useEffect, useMemo, useState } from "react";
import { Surface } from "gl-react-dom";
import { GLSL, LinearCopy, Node, Shaders, Uniform } from "gl-react";
import generateVariables, { getPerf } from "./variables";

const Main = ({ width, height, random }) => {
  const variables = useVariables({ random });
  useEffect(() => {
    window.$fxhashFeatures = generateVariables.inferProps(variables);
  }, [variables]);
  return (
    <Surface width={width} height={height}>
      <LinearCopy>
        <Post variables={variables} />
      </LinearCopy>
    </Surface>
  );
};

function useVariables({ random }) {
  return useMemo(
    () =>
      generateVariables(
        random,
        window.fxhash,
        new URLSearchParams(window.location.search).get("debug") === "1"
      ),
    []
  );
}

function useTime(ready) {
  const [time, setTime] = useState(0);
  useEffect(() => {
    if (!ready) return;
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, [ready]);
  return time;
}

const Post = ({ size, children, variables: { opts, props, symmetry } }) => {
  const time = useTime(true);

  const shader = useMemo(() => {
    const { Color, Shape } = props;
    const defs = [symmetry && "SYMMETRY"].filter(Boolean);

    const shapesShaders = {
      Square: `l += max(abs(p.x), abs(p.y));`,
      Circle: `l += length(p);`,
      X: `l += p.x;`,
      Y: `l += p.y;`,
      Cross: `l += min(abs(p.x), abs(p.y));`,
    };

    const colorsShaders = {
      Harlequin: `return fract(t)*palette(
      v+dt,
      vec3(0.7),
      vec3(0.6),
      vec3(2.0 + 8.0 * s1 * s2),
      vec3(0.2, 0.4, 0.6)
    );`,

      Green: `return palette(
      v+dt,
      vec3(0.6),
      vec3(0.4),
      vec3(0.2 + 0.2 * s1),
      vec3(0.6, 1.0, 0.2)
    );`,

      Pink: `return palette(
      v+dt,
      vec3(0.5),
      vec3(0.5),
      vec3(0.3-0.15*s1),
      vec3(0.88, 0.55, 0.64)
    );`,

      Sand: `return palette(
      v+dt,
      vec3(0.5),
      vec3(0.5),
      vec3(0.2),
      vec3(0.9, 0.7, 0.6)
    );`,

      Red: `return palette(
      v,
      vec3(0.6),
      vec3(0.6),
      vec3(1.0),
      vec3(0.2, 0.35, 0.4)
    );`,

      Dark: `return v*palette(
      v+dt,
      vec3(1.0),
      vec3(1.0),
      vec3(1.0),
      vec3(0.2, 0.4, 0.6)
    );`,
    };

    return {
      frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform float time;
  uniform vec2 resolution;
  
  uniform float shaping, crazyness, dysymmetry, cloudy, scale, tpow, dt, s1, s2, s3;
  
  #define PI ${Math.PI}
  ${defs.map((n) => `#define ${n}`).join("/n")}
  
  float hash(float p) {
    p = fract(p * .1031);
    p *= p + 33.33;
    p *= p + p;
    return fract(p);
  }
  float hash(vec2 p) {
    vec3 p3  = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
  }
  float noise(float x) {
    float i = floor(x);
    float f = fract(x);
    float u = f * f * (3.0 - 2.0 * f);
    return mix(hash(i), hash(i + 1.0), u);
  }
  float noise(vec2 x) {
    vec2 i = floor(x);
    vec2 f = fract(x);
    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
  }
  
  const mat2 m2 = mat2( 0.6,  0.8, -0.8,  0.6 );
  float fbm( in vec2 x ) {
    float f = 2.0;
    float s = 0.55;
    float a = 0.0;
    float b = 0.5;
    for( int i=0; i<9; i++ ) {
      float n = noise(x);
      a += b * n;
      b *= s;
      x = f * x;
    }
    return a;
  }
  void pR(inout vec2 p, float a) {
    p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
  }
  vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
    return a+b*cos(6.28318*(c*t+d));
  }
  vec3 pal(float t){
    float v = pow(fract(t), tpow);
    ${colorsShaders[Color]}
    return vec3(v);
  }
  
  float ease(float t) {
    return t < 0.5
      ? 4.0 * t * t * t
      : 0.5 * pow(2.0 * t - 2.0, 3.0) + 1.0;
  }

  float scene(in vec2 p, float t) {
    float dx = p.x;
    #ifdef SYMMETRY
    p = vec2(abs(p.x), p.y);
    #endif
    vec2 q = vec2( fbm( p ), fbm( p + vec2(-80.15,10.32)*s1 ) );
    vec2 r = vec2( fbm( 4.1 * q ),
    fbm( vec2(2.1 * q.x, 5.9*q.y) + vec2(40.71, 50.65)*s2 + vec2(cos(0.2 * t), sin(0.2 * t)) ) );
    float l = 0.0;
    ${shapesShaders[Shape]}
    float v = dysymmetry*dx + scale * l + cloudy * fbm(77.7*s3 + shaping * p + crazyness * r) - ease(fract(0.2 * t + dt));
    return v + 0.1 * fbm(88.*q + vec2(-77.7, 33.3));
  }
  
  void main() {
    vec2 ratio = resolution / min(resolution.x, resolution.y);
    vec3 c = vec3(0.);
    vec2 base = (uv - 0.5) * ratio;
    for (float x=-.5; x<=.5; x += 1.) {
      for (float y=-.5; y<=.5; y += 1.) {
        vec2 d = 0.5 * vec2(x,y) / resolution;
        vec2 p = base + d;
        c += pal(scene(p, time));
      }
    }
    c /= 4.0;
    gl_FragColor = vec4(c, 1.0);
  }
  `,
    };
  }, [props]);

  return (
    <Node
      shader={shader}
      uniforms={{
        time,
        ...opts,
        resolution: Uniform.Resolution,
      }}
    />
  );
};

export default Main;
