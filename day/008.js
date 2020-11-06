import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

export const n = 8;
export const title = "moontains";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL/* glsl */ `
precision highp float;
varying vec2 uv;
uniform float time;

//////// https://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdCone( in vec3 p, in vec2 c, float h )
{
  // c is the sin/cos of the angle, h is height
  // Alternatively pass q instead of (c,h),
  // which is the point at the base in 2D
  vec2 q = h*vec2(c.x/c.y,-1.0);
    
  vec2 w = vec2( length(p.xz), p.y );
  vec2 a = w - q*clamp( dot(w,q)/dot(q,q), 0.0, 1.0 );
  vec2 b = w - q*vec2( clamp( w.x/q.x, 0.0, 1.0 ), 1.0 );
  float k = sign( q.y );
  float d = min(dot( a, a ),dot(b, b));
  float s = max( k*(w.x*q.y-w.y*q.x),k*(w.y-q.y)  );
  return sqrt(d)*sign(s);
}
// https://iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
/////////

vec3 clr (float f) {
  return palette(
    f,
    vec3(0.5),
    vec3(0.5),
    vec3(1.00, 1.00, 1.00),
    //vec3(0.3, 0.6, 1.)
    vec3(0.3, 0.15, 1.)
  );
}

float SDF(vec3 p) {
  p.z += 4.;
  p.y -= 2.;
  p.x += 4. + .2 * cos(2. * time);
  float s = sdCone(p, vec2(1. + .1 *  cos(p.z*8. - 5.*time), 2.), 6.) - .5;
  p.x -= 3.;
  s = min(s, sdCone(p, vec2(1. + .1 *  cos(p.z*13. - 9.*time), 2.), 6.)-.8);
  p.x -= 3.;
  s = min(s, sdCone(p, vec2(1. + .1 *  sin(p.z*11. - 7.*time), 2.), 6.)-.9);
  p.x -= 3.;
  s = min(s, sdCone(p, vec2(1. + .1 *  sin(p.z*10. - 5.*time), 2.), 6.)-.5);
  p.x -= -15. + 20. * fract(.1 * time);
  p.z -= 3.;
  p.y -= 5. + .3 * cos(2. * time);
  s = min(s, max(0.6-length(p-vec3(0.3, 0.1, 0.1)), length(p)-0.6));
  return s;
}

void main() {
  vec3 p = vec3 (0., 0., -10.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  float shad = 1.;
  for (int i=0; i<60; i++) {
    float d = SDF(p);
    if (d<0.001) {
      shad = float(i)/60.;
      break;
    }
    p += d * dir * 0.5;
  }
  
  // Coloring
  vec3 c =
    clr(p.z * 0.2) *
    pow(smoothstep(50., 0., p.z), 3.) *
    vec3(sqrt(1. - shad));
  gl_FragColor = vec4(c,1.0);
}
`,
  },
});
