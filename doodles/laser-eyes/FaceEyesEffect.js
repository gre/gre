import React from "react";
import { Node, Shaders, GLSL, Uniform } from "gl-react";

const shaders = Shaders.create({
  loading: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform sampler2D children;
uniform float time;

vec4 shape (vec2 p) {
    p += 0.01 * cos(time) * vec2(
      cos(10. * p.x + time),
      sin(10. * p.y + time)
    );
    vec4 c = texture2D(children, p);
    c += 0.5 * fract(p.x - 2. * time);
    return c;
}
void main () {
    gl_FragColor = shape(uv);
}
        `,
  },
  main: {
    frag: GLSL`
      precision highp float;
      varying vec2 uv;
      uniform sampler2D children;
      uniform vec2 resolution;
      uniform vec4 eyes;
      uniform float time;
      uniform float mod1, mod2, mod3;
      
      #define PI ${Math.PI}
      
      void pR(inout vec2 p, float a) {
        p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
      }
      float sdCircle( vec2 p, float r ) {
        return length(p) - r;
      }
      float sdSegment( in vec2 p, in vec2 a, in vec2 b ) {
        vec2 pa = p-a, ba = b-a;
        float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
        return length( pa - ba*h );
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
      float laser (vec2 c, float scale, float ang, float i) {
        pR(c, ang);
        c *= resolution / resolution.y;
        float dist = sdCircle(c * vec2(1.0, 2.0), scale);
        float s = 4. * pow(smoothstep(0.0, -scale * (0.8 + 0.1 * cos(4. * time)), dist), 8.0);
        float speed = 1. / (mod1 + 0.1);
        pR(c, speed * i * time + .2 * i);
        vec2 a = c;
        float x = pModPolar(a, 16. * (0.1 + pow(mod1, 2.)));
        float l = (1. + 2. * fract(x * (288.259 + 56. * mod1))) * mod2;
        s += max(0., 2. * mod2 - 8. * length(a)) * smoothstep(0.01, 0.0, sdSegment(a, vec2(0.0, 0.0), vec2(l, 0.0)));
        vec2 b = c;
        speed *= -1.2 + fract(5.4 * l);
        pR(b, speed * i * time);
        x = pModPolar(b, 100. * pow(mod1, 4.));
        l = (6. * fract(56. * l + x * 4578.7459) + 2.) * mod2;
        s += max(0., 2. * mod2 - 6. * length(b)) * smoothstep(0.005, 0.0, sdSegment(b, vec2(0.0, 0.0), vec2(l, 0.0)));
        return s;
      }
      vec4 shape (vec2 p) {
          vec4 c = texture2D(children, p);
          vec2 dp = eyes.xy - eyes.zw;
          float d = length(dp) * mod2;
          float a = atan(dp.y, dp.x);
          float s =
            laser(p - eyes.xy, d, a, -1.0) +
            laser(p - eyes.zw, d, a, 1.0);
          vec4 lasC = mix(
            mix(vec4(1.0, 0., 0., 1.0), vec4(1.0, 0.3, 0.1, 1.0), 0.5 * s),
            mix(vec4(0., 0.5, 1., 1.0), vec4(0.3, 0.8, 1.0, 1.0), 0.5 * s),
            mod3
          );
          return mix(c, lasC, min(s, 1.0));
      }
      void main () {
          gl_FragColor = shape(uv);
      }
              `,
  },
});

export function FaceEyesEffect({
  children,
  eyesRes: { found, eyes },
  time,
  mod1,
  mod2,
  mod3,
}) {
  if (!found) {
    return (
      <Node
        shader={shaders.loading}
        uniforms={{
          children,
          time,
        }}
      />
    );
  }
  return (
    <Node
      shader={shaders.main}
      uniforms={{
        children,
        eyes,
        time,
        resolution: Uniform.Resolution,
        mod1,
        mod2,
        mod3,
      }}
    />
  );
}
