import React, { useState, useEffect, useRef } from "react";
import ReactDOM from "react-dom";
import { Shaders, Node, GLSL, Uniform, LinearCopy } from "gl-react";
import { Surface } from "gl-react-dom";
import { useMove } from "@use-gesture/react";
import useDimensions from "react-cool-dimensions";

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

const Persistence = ({ children: t, persistence }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence }}
  />
);

const MainShader = ({ time, n, xy: [x, y] }) => (
  <Node
    shader={shaders.main}
    uniforms={{
      resolution: Uniform.Resolution,
      time,
      clr: n % 6,
      origin: [1.6 * (x - 0.5), 1 - 1.4 * y, -2],
    }}
  />
);

export const Scene = ({ time, n, xy }) => {
  return (
    <LinearCopy>
      <Persistence persistence={0.4}>
        <MainShader time={time} n={n} xy={xy} />
      </Persistence>
    </LinearCopy>
  );
};

const Main = () => {
  const time = useTime();
  const [n, setN] = useState(0);
  const { observe, width, height } = useDimensions({});
  const ref = useRef();
  const [xy, setXY] = useState([0.5, 0.5]);

  useMove(
    (s) => {
      setXY([s.xy[0] / width, s.xy[1] / height]);
    },
    {
      target: ref,
    }
  );
  function onClick() {
    setN((n) => n + 1);
  }
  return (
    <div
      onClick={onClick}
      ref={(el) => {
        observe(el); // Set the target element for measuring
        ref.current = el; // Share the element for other purposes
      }}
      style={{ cursor: "pointer", width: "100vw", height: "100vh" }}
    >
      <Surface width={width} height={height}>
        <Scene time={time} n={n} xy={xy} />
      </Surface>
    </div>
  );
};

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
  precision highp float;
  varying vec2 uv;
  uniform sampler2D t, back;
  uniform float persistence;
  void main () {
    gl_FragColor =
      mix(
        texture2D(t, uv),
        texture2D(back, uv),
        persistence
      );
  }
      `,
  },
  main: {
    frag: GLSL`

    
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
uniform float time;
uniform float clr;
uniform vec3 origin;

#define PI ${Math.PI}


#define HIT vec2
HIT map (vec3 p);
vec3 shade (HIT m, vec3 p);
vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir);
float specularStrength (float m);

const vec3 SaffronYellow = vec3(0.96, 0.66, 0.31);
const vec3 FlamingoPink = vec3(0.85, 0.63, 0.65);
const vec3 JadeGreen = vec3(0.73, 0.81, 0.67);
const vec3 LagoonBlue = vec3(0.49, 0.73, 0.71);

float sdCylinder( vec3 p, vec3 c ) {
  return length(p.xz-c.xy)-c.z;
}
float sdCappedCylinder( vec3 p, float h, float r )
{
  vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(h,r);
  return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}
float sdSegment (in vec3 p, in float L, in float R) {
  p.y -= min(L, max(0.0, p.y));
  return length(p) - R;
}
float sdBox( vec3 p, vec3 b ) {
  vec3 q = abs(p) - b;
  return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
}
float pModInterval1(inout float p, float size, float start, float stop) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p+halfsize, size) - halfsize;
	if (c > stop) { //yes, this might not be the best thing numerically.
		p += size*(c - stop);
		c = stop;
	}
	if (c <start) {
		p += size*(c - start);
		c = start;
	}
	return c;
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
float fOpUnionSoft(float a, float b, float r) {
	float e = max(r - abs(a - b), 0.);
	return min(a, b) - e*e*0.25/r;
}

mat3 lookAt (vec3 ro, vec3 ta) {
  float cr = 0.;
  vec3 ww = normalize( ta - ro );
  vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
  vec3 vv =          ( cross(uu,ww));
  return mat3(uu,vv,ww);
}

vec3 normal (in vec3 p) {
	vec3 eps = vec3(0.0005, 0.0, 0.0);
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

// https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm
float softshadow( in vec3 ro, in vec3 rd, float mint, float maxt, float k ) {
  float res = 1.0;
  float ph = 1e20;
  float t = mint;
  for (int i=0; i<40; i++) {
    float h = 0.5 * map(ro + rd*t).x;
    if (t>=maxt) break;
    if( h<0.001) return 0.0;
    float y = h*h/(2.0*ph);
    float d = sqrt(h*h-y*y);
    res = min( res, k*d/max(0.0,t-y) );
    ph = h;
    t += h;
  }
  return res;
}

HIT marcher (inout vec3 p, vec3 dir) {
  HIT hit = HIT(0., 2.0);
  float t = 0.;
  for (int i=0; i<80; i++) {
    HIT h = map(p + t * dir);
    t += min(.3, h.x);
    if (abs(h.x) < .0001) {
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

float specular (vec3 n, float m, vec3 ldir, vec3 dir, float p) {
  return specularStrength(m) * pow(max(dot(dir, reflect(ldir, n)), 0.0), p);
}

vec3 lighting (HIT hit, vec3 p, vec3 n, vec3 dir) {
  vec3 l, ldir;
  vec3 c = vec3(0.);
  l = vec3(-2., 1.5, -3.4);
  ldir = normalize(l - p);
  c +=
  vec3(0.9, 0.7, 0.5) * (
    // ambient
    0.1
    // diffuse
    + shade(hit, p)
      * (.5 + .5 * diffuse(p, n, l)) // half lambert
      * (0.5 + 0.5 * softshadow(p, ldir, 0.05, 5., 20.))
    + specular(n, hit.y, ldir, dir, 10.)
  );
  l = vec3(2., 5., -2.);
  ldir = normalize(l - p);
  c +=
  vec3(0.3, 0.5, 0.6) * (
  // ambient
  0.1
  // diffuse
  + shade(hit, p)
    * (.5 + .5 * diffuse(p, n, l)) // half lambert
  + specular(n, hit.y, ldir, dir, 20.)
  );
  return c;
}

float specularStrength (float m) {
  return 0.2;
}

vec3 shade (HIT hit, vec3 _p) {
  if (hit.y < 1.0) return vec3(1.0, 0.0, 0.0);
  if (hit.y < 2.0) return vec3(.8);
  if (hit.y < 3.0) {
    if(hit.y < 2.01) {
      return vec3(0.1);
    }
    if(hit.y < 2.02) {
      return vec3(0.9);
    }
    if(hit.y < 2.03) {
      return SaffronYellow;
    }
    if(hit.y < 2.04) {
      return FlamingoPink;
    }
    if(hit.y < 2.05) {
      return JadeGreen;
    }
    if(hit.y < 2.06) {
      return LagoonBlue;
    }
    if (hit.y < 2.2) {
      return mix(
        vec3(0.01, 0.01, 0.05),
        vec3(0.0, 0.8, 2.0),
        step(2.15, hit.y)
      );
    }
    return vec3(0.7);
  }
  return vec3(0.0);
}

// ref: 5mm -> 0.1
HIT sdLedgerNanoS (vec3 p, float rot, float active, float clr) {
  float btn = sdSegment(vec3(p.y - 0.165, abs(p.x - 0.14) - 0.2, p.z), 0.08, 0.02);
  HIT s = HIT(
    min(
      max(
        sdBox(p, vec3(0.56, 0.16, 0.05)) - 0.01, // main casing
        -min(
          sdBox(p-vec3(0.15, 0.0, -0.06), vec3(0.27, 0.08, 0.06)), // screen carving
          min(
            sdCylinder(p.xzy, vec3(-0.4, 0.0, 0.07)), // swivel hook carving
            btn-0.01 // btns carving
          )
        )
      ),
      btn
    ),
    2.0 + 0.01 * clr);
  // screen
  s = opU(s, HIT(sdBox(p-vec3(0.15, 0.0, 0.0), vec3(0.28, 0.08, 0.03)), 2.1 + 0.06 * active));
  // swivel
  p.x += 0.4;
  pR(p.xy, rot);
  p.x -= 0.4;
  float x = p.x + 0.8;
  float z = abs(p.z) - 0.07;
  float swivel = min(
    max(
      min(
        sdCappedCylinder(vec3(p.y, z, x - 0.4), 0.17, 0.01),
        sdBox(vec3(x + 0.1, p.y, z), vec3(0.5, 0.17, 0.01))
      ),
      -sdCylinder(p.xzy, vec3(-0.4, 0.0, 0.1))
    ),
    sdBox(vec3(x + 0.6, p.y, p.z), vec3(0.01, 0.17, 0.07))
  );
  s = opU(s, HIT(swivel, 2.2));
  return s;
}

float cubicInOut(float t) {
  return t < 0.5
    ? 4.0 * t * t * t
    : 0.5 * pow(2.0 * t - 2.0, 3.0) + 1.0;
}

HIT map (vec3 p) {
  HIT s = HIT(10. - length(p), 0.);
  p.x += 0.5;
  float f = pModInterval1(p.z, 0.8, -1.0, 20.0);
  p.x -= 0.5;
  float t = 3. * fract(0.1 * f + 0.5 * time);
  float active = step(0.9, t) * step(t, 1.6);
  float rot = PI * (
    1. +
    cubicInOut(min(1.0, t)) +
    cubicInOut(min(1.0, max(t - 1.5, 0.0)))
  );
  pR(p.xy, 0.1 * f * cos(0.5 * PI * time));
  s = opU(s, sdLedgerNanoS(p, rot, active, clr));
  return s;
}

vec3 scene(vec2 uv) {
  vec3 focus = vec3(0.0, 0.0, 0.);
  vec3 c = vec3(0.);
  vec3 dir = normalize(vec3(uv - .5, 1.));
  dir = lookAt(origin, focus) * dir;
  vec3 p = origin;
  HIT hit = marcher(p, dir);
  vec3 n = normal(p);
  c += lighting(hit, p, n, dir);
  c = mix(c, vec3(0.9), pow(smoothstep(4., 10., length(p-origin)), .5));
  return c;
}

vec3 render() {
  vec3 c = vec3(0.);
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 base = 0.5 + (uv - 0.5) * ratio;
  c += scene(base);
  return c;
}

void main() {
  vec3 c = render();
  gl_FragColor = vec4(c, 1.0);
}

  `,
  },
});

ReactDOM.render(<Main />, document.getElementById("main"));
