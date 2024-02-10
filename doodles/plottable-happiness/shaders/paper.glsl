precision highp float;
varying vec2 uv;
uniform vec2 resolution;
uniform float grain, seed;

void pR(inout vec2 p, float a) {
  p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
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
const mat2 m2 = mat2( 0.4,  0.7, -0.7,  0.4 );
float fbm( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<3; i++ ) {
    float n = noise(x);
    a += b * n;
    b *= s;
    x = f * x;
  }
  return a;
}
float paper(vec2 p, float z, float seed) {
  pR(p, seed);
  p += seed;
  float n = 0.7 * smoothstep(-0.1, 0.2, abs(fbm(-3.0 * p * z - seed)-0.5)-0.01) +
  0.3 * smoothstep(0.0, 0.1, abs(fbm(-6.0 * p * z + seed)-0.5)-0.01);
  return n;
}

void main () {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 p = 0.5 + (uv - 0.5) * ratio;
  float t = paper(p, grain, seed);
  gl_FragColor = vec4(t, t, t, 1.0);
}