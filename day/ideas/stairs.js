import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";
import { useGamepadFreeControls } from "../hooks/useGamepadFreeControls";

export const n = 23;
export const title = "";

export const Shader = ({ time }) => {
  const { origin, rotation, buttonsPressCount } = useGamepadFreeControls();
  return (
    <Node
      shader={shaders.node}
      uniforms={{
        cameraMode: buttonsPressCount[0] % 2,
        time,
        origin,
        rotation,
      }}
    />
  );
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

const float PI = ${Math.PI};

float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}

// Repeat only a few times: from indices <start> to <stop> (similar to above, but more flexible)
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

// Repeat around the origin by a fixed angle.
// For easier use, num of repetitions is use to specify the angle.
float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2. * PI / repetitions;
	float a = atan(p.y, p.x) + angle/2.;
	float r = length(p);
	float c = floor(a/angle);
	a = mod(a,angle) - angle/2.;
	p = vec2(cos(a), sin(a))*r;
	// For an odd number of repetitions, fix cell index of the cell in -x direction
	// (cell index would be e.g. -5 and 5 in the two halves of the cell):
	if (abs(c) >= (repetitions/2.)) c = abs(c);
	return c;
}

mat2 rot (float a) {
  float c = cos(a);
  float s = sin(a);
  return mat2(c,s,-s,c);
}

float sphere (vec3 p, float r) {
  return length(p)-r;
}

float box (vec3 p, vec3 c) {
  return length(max(abs(p)-c,0.));
}

float smin( float a, float b, float k ) {
  float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );
  return mix( b, a, h ) - k*h*(1.0-h);
}

float smax(float a,float b, float k) {
    return -smin(-a,-b,k);
}

float stairs (vec3 p, float i) {
/*
  float u = -1.;

  p.x -= -3.;
  for (float f=-3.; f<=3.; f+=1.) {
  */
    float id = pModInterval1(p.x, 1., -3., 3.);

/*
    float id = f;
    p.x -= 1.;
    */

    float height = 5. - i * 1.15;
    float offset = .1 * id - i * .575;
    float width = .5;// + .2 + .2 * i - .1 * p.y;
    float shape = box(p + vec3(0., offset, 0.), vec3(.5, height - offset, width));
    return shape;
/*
u = min(u, shape);

  }
  return u;
  */
}

float map(vec3 p) {
  float s = 99.;
  // camera
  p.y += 4.;
  p.z -= 20.;
  p.x -= 0.;
  //p.z -= 11.;
  p.yz *= rot(-PI/4.);
  p.xz *= rot(PI/4.);

  for (float i = 0.; i < 4.; i++) {
    s = min(s, stairs(p, i));
    p.x -= 3.;
    p.z -= 3.;
    p.y += 1.15;
    p.xz *= rot(PI/2.);
  }
  return s;
}

/*
vec3 calcNormal(vec3 pos, float eps) {
  const vec3 v1 = vec3( 1.0,-1.0,-1.0);
  const vec3 v2 = vec3(-1.0,-1.0, 1.0);
  const vec3 v3 = vec3(-1.0, 1.0,-1.0);
  const vec3 v4 = vec3( 1.0, 1.0, 1.0);
  return normalize( v1 * map( pos + v1*eps ) +
                    v2 * map( pos + v2*eps ) +
                    v3 * map( pos + v3*eps ) +
                    v4 * map( pos + v4*eps ) );
}
*/

vec3 calcNormal( in vec3 pos ) {
  vec2 e = vec2(1.0,-1.0)*0.5773*0.0005;
  return normalize( e.xyy*map( pos + e.xyy ) +
          e.yyx*map( pos + e.yyx ) +
          e.yxy*map( pos + e.yxy ) +
          e.xxx*map( pos + e.xxx ));
}

void main() {
  vec3 p, dir;
  if (cameraMode) {
    p = origin + 25. * rotation * vec3(2. * (uv - .5), 0.);
    dir = rotation * vec3(0., 0., 1.);
  }
  else {
    p = origin;
    dir = rotation * vec3(2. * (uv-.5), 2.);
  }
  float shad = 1.;
  for (int i=0; i<100; i++) {
    float d = map(p);
    if (d<0.001) {
      shad = float(i)/100.;
      break;
    }
    p += d * dir * 0.5;
  }

  vec3 nor = calcNormal(p);
  vec3 ref = reflect(dir, nor);
  vec3 lig = normalize(dir);
  float dif = clamp(1. + dot( nor, lig ), 0.0, 1.0 );
  vec3 c = vec3(step(shad, 0.999)) * mix(dif, 1., 0.5);
  gl_FragColor = vec4(c,1.0);
}`,
  },
});
