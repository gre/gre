import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 24;
export const title = "dark cubes";

export const Shader = ({ time }) => {
  return <Node shader={shaders.node} uniforms={{ time }} />;
};

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

uniform bool cameraMode;
uniform vec3 origin;
uniform mat3 rotation;

#define MAX_DIST 	60.
#define MIN_DIST	.001

const float PI = ${Math.PI};
const float PI2 = ${2 * Math.PI};

vec2 opU (vec2 a, vec2 b) {
  return (a.x<b.x) ? a : b;
}
vec2 opI (vec2 a, vec2 b) {
  return (a.x>b.x) ? a : b;
}

float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}

float vmax(vec3 v) {
  return max(max(v.x, v.y), v.z);
}

float fBox(vec3 p, vec3 b, float r) {
	vec3 d = abs(p) - b;
  return length(max(d, vec3(0))) + vmax(min(d, vec3(0)))-r;
}
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}

vec2 shape (vec3 p, float a, float b) {
  float m = abs(2. + .3 * a + .4 * b);
  m *= step(2., mod(2.7 * a + 13.3 * b, 4.));
  vec2 o = vec2(fBox(p, vec3(1.), .0), m);
  pR(p.xy, .05 * time + .2 * a);
  pR(p.yz, -.2 * time - .4 * b);
  pR(p.xz, .4 * time + .1 * a - b);
  p *= 1. + .5 * cos(2. * time + .1 * a);
  p.xy += .2 * vec2(cos(.2 * time - a * 3.), sin(.3 * time + b));
  o = opI(o, vec2(fBox(p, vec3(1.), .0), m));
  return o;
}

vec2 map(vec3 p) {
  vec2 o = vec2(MAX_DIST, 0.);
  p.x -= 1. + .5 * time;
  p.y += 14. * sqrt(smoothstep(20., 2., time));
  pR(p.yz, PI/6.);
  pR(p.xz, .05 * time);
  float a = pMod1(p.z, 4.);
  float b = pMod1(p.x, 5.);
  o = opU(o, vec2(p.y, 1.));
  p.y -= 1.8 + .7 * cos(2. * time + 2. * (b + a));
  o = opU(o, shape(p, a, b));
  return o;
}


// Boilerplate inspired from https://www.shadertoy.com/view/WscBDS

// Tetrahedron technique @iq
// https://www.iquilezles.org/www/articles/normalsSDF
vec3 getNormal(vec3 p, float t){
    float e = MIN_DIST *t;
    vec2 h = vec2(1.,-1.)*.57735027;
    return normalize( h.xyy*map( p + h.xyy*e ).x +
					  h.yyx*map( p + h.yyx*e ).x +
					  h.yxy*map( p + h.yxy*e ).x +
					  h.xxx*map( p + h.xxx*e ).x );
}

vec2 marcher(vec3 ro, vec3 rd, int maxsteps) {
	float d = 0.;
    float m = -1.;
    for(int i=0; i<300; i++){
      if (i>maxsteps) break;
    	vec2 t = map(ro + rd * d);
        if(abs(t.x)<d*MIN_DIST||d>MAX_DIST) break;
        d += i<64 ? t.x*.45 : t.x * .85;
        m  = t.y;
    }
	return vec2(d,m);
}

float getDiff(vec3 p, vec3 n, vec3 lpos) {
    vec3 l = normalize(lpos-p);
    float dif = clamp(dot(n,l),.01 , 1.);
    float shadow = marcher(p + n * .01, l, 128).x;
    if(shadow < length(p -  lpos)) dif *= .25;
    return dif;
}

vec3 camera(vec3 lp, vec3 ro, vec2 uv) {
	vec3 cf = normalize(lp - ro),
         cr = normalize(cross(vec3(0,1,0),cf)),
         cu = normalize(cross(cf,cr)),
         c  = ro + cf *.85,
         i  = c + uv.x * cr + uv.y * cu,
         rd = i - ro;
    return rd;
}


vec3 shp;
vec2 sid,sip,bid;
float saveHash, ti, tf, tg;
vec3 thp;
vec2 tip,fid;
float thsh;

// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 color (float t) {
  return clamp(palette(
    t,
    vec3(.4),
    vec3(.9),
    vec3(.3, .5, .6),
    vec3(1., .6, .4)
  ), 0., 1.);
}

vec3 getColor(float m) {
  if (m < 2.) return vec3(.0);
  return color(m*.2);
}

vec4 mainImage() {
    vec3 C = vec3(0.),
        FC =  vec3(.2);
    vec3 lp = vec3(0.,0.,0.),
         ro = vec3(2.25,2.15,11.5);
	vec3 rd = camera(lp,ro,uv-.5);
  vec2 t = marcher(ro,rd, 256);
  thsh = saveHash;
  thp = shp;
	tip = sip;

  if(t.x<MAX_DIST){
  	vec3 p = ro + rd * t.x;
  	vec3 n = getNormal(p, t.x);
      vec3 lpos = vec3(1.5,2.5, 16.25);
  	float diff = getDiff(p, n, lpos);
		vec3 h = getColor(t.y);
      C+=diff * h;
      if(t.y>0.){
      	vec3 rr=reflect(rd,n);
          vec2 tr = marcher(p ,rr, 192);
		thsh = saveHash;
          thp = shp;
          tip = sid;
          if(tr.x<MAX_DIST){
              p += rr*tr.x;
              n = getNormal(p,tr.x);
              diff = getDiff(p,n,lpos);
              h = max(getColor(tr.y),FC);
              C+=(diff * h)*.4;

              if(t.y>0.){
                  rr=reflect(rr,n);
                  tr = marcher(p ,rr, 192);
                  thsh = saveHash;
                  thp = shp;
                  tip = sid;
                  if(tr.x<MAX_DIST){
                      p += rr*tr.x;
                      n = getNormal(p,tr.x);
                      diff = getDiff(p,n,lpos);
                      h = max(getColor(tr.y),FC);
                      C+=(diff * h)*.4;
                  }
              }

          }
      }
    }
    C = mix( C, FC, 1.-exp(-.00015*t.x*t.x*t.x));
    return vec4(pow(C, vec3(0.4545)),1.0);
}

void main() {
  gl_FragColor = mainImage();
}`,
  },
});
