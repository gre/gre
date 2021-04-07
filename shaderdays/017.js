

import { Shaders, Node, GLSL } from "gl-react";

// rendering engine inspired from https://www.shadertoy.com/view/Xds3zN
// primitives https://mercury.sexy/hg_sdf/

export const n = 17;
export const title = "screws";

export const Shader = ({ time }) => (
  <Node shader={shaders.node} uniforms={{ time }} />
);

const shaders = Shaders.create({
  node: {
    frag: GLSL`
precision highp float;
varying vec2 uv;
uniform float time;

const float PI = ${Math.PI};

float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
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

float fCone(vec3 p, float radius, float height) {
	vec2 q = vec2(length(p.xz), p.y);
	vec2 tip = q - vec2(0, height);
	vec2 mantleDir = normalize(vec2(height, radius));
	float mantle = dot(tip, mantleDir);
	float d = max(mantle, -q.y);
	float projected = dot(tip, vec2(mantleDir.y, -mantleDir.x));

	// distance to tip
	if ((q.y > height) && (projected < 0.)) {
		d = max(d, length(tip));
	}

	// distance to base ring
	if ((q.x > radius) && (projected > length(vec2(height, radius)))) {
		d = max(d, length(q - vec2(radius, 0.)));
	}
	return d;
}

float pModPolar(inout vec2 p, float repetitions) {
	float angle = 2.*PI/repetitions;
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

float fCylinder(vec3 p, float r, float height) {
	float d = length(p.xz) - r;
	d = max(d, abs(p.y) - height);
	return d;
}

float sphere (vec3 p, float r) {
  return length(p)-r;
}

float box (vec3 p, vec3 c) {
  return length(max(abs(p)-c,0.));
}

float opU( float d1, float d2 ) {
	return min(d1, d2);
}

float opD( float d1, float d2 ) {
	return max(d1, -d2);
}

float opSmoothSubtraction( float d1, float d2, float k ) {
    float h = clamp( 0.5 - 0.5*(d2+d1)/k, 0.0, 1.0 );
    return mix( d2, -d1, h ) + k*h*(1.0-h); }

float random (vec2 st) {
  return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

void opDig (inout vec3 p, float dig, float k) {
  p.y -= 1.5;
  p.y += dig * 2.2;
  pR(p.xz, dig * k * .5 * PI);
}

float sdScrewHead (vec3 p, vec2 id, float screwW) {
  float r1 = random(id * .01);
  float w = .4;
  float h = .02 * (1. + mod(floor(r1 * 13.), 2.));
  float h2 = .3 * step(r1, .5);
  float sw = 0.5 + 0.5 * step(r1, .3);
  p.y -= 1.;
  vec3 pcut = p - vec3(.0, .1, 0.);
  float cut = box(pcut, vec3(.5 * sw, .04, .02));
  pcut.x -= 10. * step(r1, .7);
  pR(pcut.xz, PI/2.);
  cut = opU(cut, box(pcut, vec3(.5, .04, .02)));
  float head = fCylinder(p + vec3(.0, h2, 0.), 0.12 + screwW, h2);

  if (mod(29. * r1, 7.) < 1.) {
    pModPolar(p.xz, 6.);
    h += .02;
    p.y -= h;
    head = opU(head, box(p, vec3(w * .7, h, w * .7)));
    p.y += h;
    h *= 0.4;
  }
  else {
    head = opU(head, opD(sphere(vec3(1., 4., 1.) * p, w), p.y));
  }
  head = opU(head, fCylinder(p + vec3(.0, h/2., 0.), w, h));

  float s = opSmoothSubtraction(cut, head, .05);
  return s;
}

float sdScrew (vec3 p, float w, float k) {
  float bottom = fCone((p + vec3(.0, 1., .0)) * vec3(1., -1., 1.), w, 3. * w);
  float c = cos(k*p.y);
  float s = sin(k*p.y);
  mat2  m = mat2(c,-s,s,c);
  p.xz *= m;
  return opU(
    box(p, vec3(.2, 1., w)),
    bottom
  );
}

float SDF(vec3 p) {
  // plane floor
  float res = p.y;
  // repeat
  vec2 id = vec2(
    pMod1(p.x, 2.),
    pModInterval1(p.z, 2., -2., 1.)
  );
  float dig = .5 + .5 * cos(0.5 * time * (random(id) + 0.1 * (0.7 * id.x + 2. * id.y - 3.)));
  float k = 8. + 8. * random(id * .001);
  float w = .08 + 0.04 * mod(id.x, 3.);
  // card
  float card = opD(
    box(p, vec3(.8, 0.05, .8)),
    fCylinder(p, 2. * w, .1)
  );
  res = opU(res, card);
  p.y -= .05;
  // screw
  opDig(p, dig, k);
  res = opU(res, sdScrewHead(p, id, w));
  res = opU(res, sdScrew(p, w, k));
  return res;
}

vec3 color (vec3 p) {
  vec3 col =
    vec3(.3) +
    step(.01, p.y) * vec3(.3) +
    step(.11, p.y) * mix(
      vec3(.4, .2, -0.2),
      vec3(.0),
      step(fract(0.25 + (p.x + p.z)/4.), 0.5)
    );

  return col;
}

float raycast( in vec3 ro, in vec3 rd ) {
  float res = -1.0;
  float t = 0.;
  for(int i=0; i<200; i++ ) {
    float h = SDF( ro+rd*t );
    if( abs(h)<(0.0001*t) ) {
        res = t;
        break;
    }
    t += h * .5;
  }
  return res;
}

float calcSoftshadow( in vec3 ro, in vec3 rd, in float mint, in float tmax ) {
    // bounding volume
    float res = 1.0;
    float t = mint;
    for( int i=0; i<24; i++ ) {
		float h = SDF( ro + rd*t );
        float s = clamp(8.0*h/t,0.0,1.0);
        res = min( res, s*s*(3.0-2.0*s) );
        t += clamp( h, 0.02, 0.2 );
        if( res<0.004 ) break;
    }
    return clamp( res, 0.0, 1.0 );
}

vec3 calcNormal( in vec3 pos ) {
  vec2 e = vec2(1.0,-1.0)*0.5773*0.0005;
  return normalize( e.xyy*SDF( pos + e.xyy ) +
          e.yyx*SDF( pos + e.yyx ) +
          e.yxy*SDF( pos + e.yxy ) +
          e.xxx*SDF( pos + e.xxx ));
}

float calcAO( in vec3 pos, in vec3 nor ) {
	float occ = 0.0;
  float sca = 1.0;
  for( int i=0; i<5; i++ ) {
      float h = 0.01 + 0.12*float(i)/4.0;
      float d = SDF( pos + h*nor );
      occ += (h-d)*sca;
      sca *= 0.95;
      if( occ>0.35 ) break;
  }
  return clamp( 1.0 - 3.0*occ, 0.0, 1.0 ) * (0.5+0.5*nor.y);
}

void main() {
  vec3 p = vec3(0., 0., 0.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  p.y += 3.5;
  p.x -= 1.5 - time;
  p.z -= 1.5;
  pR(dir.yz, -.9);
  pR(dir.xz, 0.5 * PI + 0.2 * cos(0.5 * time));

  float t = raycast(p, dir);
  vec3 pos = p + t * dir;
  vec3 nor = calcNormal(pos);
  vec3 ref = reflect(dir, nor);
  float occ = calcAO(pos, nor);

  vec3 lin = vec3(0.0);
  vec3 col = color(pos);
  // sun
  {
    float phase = sin(PI * time / 30.);
    vec3  lig = normalize( vec3(-0.5, 0.4 + phase, -0.7) );
    vec3  hal = normalize( lig - dir );
    float dif = clamp( dot( nor, lig ), 0.0, 1.0 );
  // if( dif>0.0001 )
	      dif *= calcSoftshadow( pos, lig, 0.02, 2.5 );
float spe = pow( clamp( dot( nor, hal ), 0.0, 1.0 ),16.0);
          spe *= dif;
          spe *= 0.04+0.96*pow(clamp(1.0-dot(hal,lig),0.0,1.0),5.0);
    lin += col*1.4*dif*vec3(1.30,1.00,0.70);
  }
  // sky
  {
    float dif = sqrt(clamp( 0.5+0.5*nor.y, 0.0, 1.0 ));
          dif *= occ;
    float spe = smoothstep( -0.2, 0.2, ref.y );
          spe *= dif;
          spe *= 0.04+0.96 * pow(clamp(1.0+dot(nor,dir),0.0,1.0), 5.0 );
          spe *= calcSoftshadow( pos, ref, 0.02, 2.5 );
    lin += col*0.9*dif*vec3(0.8,0.8,1.);
  }
  col = lin;

  gl_FragColor = vec4(clamp(col, 0.0, 1.0), 1.0);
}`,
  },
});
