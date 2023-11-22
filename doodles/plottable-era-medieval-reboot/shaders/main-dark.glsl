precision highp float;
varying vec2 uv;
uniform vec3 baseColor, background;
uniform float grainAmp, lighting, time;
uniform vec3 primary, primaryHighlight, secondary, secondaryHighlight, third, thirdHighlight;
uniform sampler2D t, paper;

vec3 pal(float t, vec3 c1, vec3 c2) {
  float m = smoothstep(0.3, 0.0, t);
  return mix(
    vec3(0.0),
    mix(c1, c2, m),
    smoothstep(1.0, 0.5, t)
  );
} 
void main() {
  vec2 p = uv;
  float gain = smoothstep(0.3, 1.0, abs(cos(3.14159*(length(p-0.5)-time))));
  vec4 g = texture2D(paper, p);
  float grain = g.r;
  vec4 v = texture2D(t, p);
  vec3 c1 = pal(v.r, primary, primaryHighlight);
  vec3 c2 = pal(v.g, secondary, secondaryHighlight);
  vec3 c3 = pal(v.b, third, thirdHighlight);
  vec3 c =
  (c1 + c2 + c3) * (1. + lighting * gain) +
  grainAmp * grain +/*developed by @greweb*/
  baseColor +
  background * smoothstep(0.5, 1.0, v.r * v.g * v.b);
  gl_FragColor = vec4(c, 1.0);
}