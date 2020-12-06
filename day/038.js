import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL, LinearCopy, Uniform } from "gl-react";

export const n = 38;
export const title = "Golden mandelbulb";

export const Shader = ({ time }) => (
  <LinearCopy>
    <Persistence persistence={0.5}>
      <Node shader={shaders.node} uniforms={{ time }} />
    </Persistence>
  </LinearCopy>
);

const Persistence = ({ children: t, persistence }) => (
  <Node
    shader={shaders.persistence}
    backbuffering
    uniforms={{ t, back: Uniform.Backbuffer, persistence }}
  />
);

const shaders = Shaders.create({
  persistence: {
    frag: GLSL`
    precision highp float;
    varying vec2 uv;
    uniform sampler2D t, back;
    uniform float persistence;
    void main () {
      gl_FragColor = mix(
        texture2D(t, uv),
        texture2D(back, uv),
        persistence
      );
    }
        `,
  },
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

vec2 map (vec3 p);


void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}

vec2 opU (vec2 a, vec2 b) {
  if (a.x < b.x) return a;
  return b;
}

#define PI ${Math.PI}

// FROM https://www.shadertoy.com/view/4sdGWN

#define HASHSCALE1 .1031

float hash(float p) {
	vec3 p3  = fract(vec3(p) * HASHSCALE1);
  p3 += dot(p3, p3.yzx + 19.19);
  return fract((p3.x + p3.y) * p3.z);
}
vec3 randomSphereDir(vec2 rnd) {
	float s = rnd.x*PI*2.;
	float t = rnd.y*2.-1.;
	return vec3(sin(s), cos(s), t) / sqrt(1.0 + t * t);
}
vec3 randomHemisphereDir(vec3 dir, float i) {
	vec3 v = randomSphereDir( vec2(hash(i+1.), hash(i+2.)) );
	return v * sign(dot(v, dir));
}

float ambientOcclusion( in vec3 p, in vec3 n, in float maxDist, in float falloff ) {
  const int nbIte = 12;
  const float nbIteInv = 1./float(nbIte);
  const float rad = 1.-1.*nbIteInv;
  float ao = 0.0;
  for( int i=0; i<nbIte; i++ ) {
    float l = hash(float(i))*maxDist;
    vec3 rd = normalize(n+randomHemisphereDir(n, l )*rad)*l;
    ao += (l - max(map( p + rd ).x, 0.)) / maxDist * falloff;
  }
  return clamp( 1.-ao*nbIteInv, 0., 1.);
}

// huge kudos to https://www.iquilezles.org/www/articles/mandelbulb/mandelbulb.htm
vec2 mandelbulb(vec3 p) {
  vec3 w = p;
  float dz = 1.;
  float m = dot(w,w);
  float index = 0.;
  for (int i=0; i<6; i++) {
    dz = 8.0*pow(sqrt(m),7.0)*dz + 1.0;
    float x = w.x; float x2 = x*x; float x4 = x2*x2;
    float y = w.y; float y2 = y*y; float y4 = y2*y2;
    float z = w.z; float z2 = z*z; float z4 = z2*z2;

    float k3 = x2 + z2;
    float k2 = inversesqrt( k3*k3*k3*k3*k3*k3*k3 );
    float k1 = x4 + y4 + z4 - 6.0*y2*z2 - 6.0*x2*y2 + 2.0*z2*x2;
    float k4 = x2 - y2 + z2;

    w.x =  64.0*x*y*z*(x2-z2)*k4*(x4-6.0*x2*z2+z4)*k1*k2;
    w.y = -16.0*y2*k3*k4*k4 + k1*k1;
    w.z = -8.0*y*k4*(x4*x4 - 28.0*x4*x2*z2 + 70.0*x4*z4 - 28.0*x2*z2*z4 + z4*z4)*k1*k2;

    w += p;

    m = dot(w,w);
    if (m>256.) break;
    index += 1.;
  }

  return vec2(0.25*log(m)*sqrt(m)/dz, 2. + index);
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

vec2 marcher (inout vec3 p, vec3 dir) {
  vec2 t = vec2(999., 0.);
  for (int i=0; i<80; i++) {
    vec2 hit = map(p);
    p += dir * hit.x;
    if (hit.x < 0.001) {
      t = hit;
      break;
    }
  }
  return t;
}

vec2 map (vec3 p) {
  vec2 s = vec2(min(p.y, 40. - p.z), 1.); // ground
  p.y -= 1.;
  pR(p.xz, .05 * time);
  pR(p.yz, -.1 * time);
  s = opU(s, mandelbulb(p));
  return s;
}

vec3 color (float t) {
  if (t < 2.) return vec3(.9);
  return palette(
    .1 * t + time * step(mod(time, 10.), 5.),
    vec3(.5),
    vec3(.5),
    vec3(1.),
    vec3(.5, .4, .2)
  );
}

void main() {
  vec3 origin = vec3(0., 3., -2.8);
  origin *= .7 + .3 * cos(.2 * time);
  vec3 clr = vec3(0.);
  // Anti aliasing
  for (float x=-.5; x<=.5; x += 1.) {
    for (float y=-.5; y<=.5; y += 1.) {
      vec2 uvP = uv;
      uvP += vec2(x, y) / 800.;
      vec3 dir = normalize(vec3(uvP - .5, 1.));
      pR(dir.yz, -.5);
      vec3 p = origin;
      vec2 hit = marcher(p, dir);
      vec3 n = normal(p);
      vec3 c = vec3(0.);
      c += color(hit.y) * vec3(1., .5, .2) * diffuse(p, n, vec3(-5., 6., -4.));
      c += color(hit.y) * vec3(.2, .5, 1.) * diffuse(p, n, vec3(5., 6., -4.));
      c += color(hit.y) * vec3(.6) * diffuse(p, n, vec3(0., 8., -2.));
      c *= ambientOcclusion(p, n, 1.8, 1.2);
      c += smoothstep(5., 10., p.z);
      clr += c;
    }
  }
  clr /= 4.;
  gl_FragColor = vec4(clr, 1.0);
}`,
  },
});
