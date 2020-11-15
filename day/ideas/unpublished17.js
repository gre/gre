import Head from "next/head";
import { Surface } from "gl-react-dom";
import { Shaders, Node, GLSL } from "gl-react";

// rendering engine inspired from https://www.shadertoy.com/view/Xds3zN

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


// http://iquilezles.org/www/articles/boxfunctions/boxfunctions.htm
vec2 iBox( in vec3 ro, in vec3 rd, in vec3 rad ) {
    vec3 m = 1.0/rd;
    vec3 n = m*ro;
    vec3 k = abs(m)*rad;
    vec3 t1 = -n - k;
    vec3 t2 = -n + k;
	return vec2( max( max( t1.x, t1.y ), t1.z ),
	             min( min( t2.x, t2.y ), t2.z ) );
}

float opRepF(in float p, in float s) {
  return mod(p+s*0.5,s)-s*0.5;
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

float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
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

float sdScrew1 (vec3 p) {
  p.z -= 1.;
  p.y -= 1.;
  pR(p.xz, time);
  float s = fCylinder(p-vec3(.0, 1., .0), 0.3, 0.03);

  {
    float k = 8.;
    float c = cos(k*p.y);
    float s = sin(k*p.y);
    mat2  m = mat2(c,-s,s,c);
    p.xz *= m;
  }
  float d = box(p, vec3(.2, 1., .1));

  s = opU(s, d);
  return s;
}

float SDF(vec3 p) {
  float res = p.y; // plane floor
  p.y -= 1.;

  res = opU(res, sdScrew1(p));
/*
  float x = pModInterval1(p.x, 5., -1., 1.);
  float y = pModInterval1(p.z, 5., 1., 3.);

  res = opU(res, sphere(p, 1.));
  */

  return res;
}

vec3 color (vec3 p) {
  vec3 col = vec3(.5);
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
    float tp = (0.8-ro.y)/rd.y; if( tp>0.0 ) tmax = min( tmax, tp );
    float res = 1.0;
    float t = mint;
    for( int i=0; i<24; i++ ) {
		float h = SDF( ro + rd*t );
        float s = clamp(8.0*h/t,0.0,1.0);
        res = min( res, s*s*(3.0-2.0*s) );
        t += clamp( h, 0.02, 0.2 );
        if( res<0.004 || t>tmax ) break;
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
  vec3 p = vec3(0., 3.5, 0.);
  vec3 dir = normalize(vec3((uv - 0.5) * 2.,1.));
  pR(dir.yz, -1.);
  p.x -= .5;
  pR(p.xz, time);

  // pR(dir.xz, cos(time));

  float t = raycast(p, dir);
  vec3 pos = p + t * dir;
  vec3 nor = calcNormal(pos);
  vec3 ref = reflect(dir, nor);
  float occ = calcAO(pos, nor);

  vec3 lin = vec3(0.0);
  vec3 col = color(pos);
  float ks = 0.;
  // sun
  {
    vec3  lig = normalize( vec3(-0.5, 0.4, -0.6) );
    vec3  hal = normalize( lig - dir );
    float dif = clamp( dot( nor, lig ), 0.0, 1.0 );
  //if( dif>0.0001 )
	      dif *= calcSoftshadow( pos, lig, 0.02, 2.5 );
float spe = pow( clamp( dot( nor, hal ), 0.0, 1.0 ),16.0);
          spe *= dif;
          spe *= 0.04+0.96*pow(clamp(1.0-dot(hal,lig),0.0,1.0),5.0);
    lin += col*2.20*dif*vec3(1.30,1.00,0.70);
    lin +=     5.00*spe*vec3(1.30,1.00,0.70)*ks;
  }
  // sky
  {
    float dif = sqrt(clamp( 0.5+0.5*nor.y, 0.0, 1.0 ));
          dif *= occ;
    float spe = smoothstep( -0.2, 0.2, ref.y );
          spe *= dif;
          spe *= 0.04+0.96*pow(clamp(1.0+dot(nor,dir),0.0,1.0), 5.0 );
          spe *= calcSoftshadow( pos, ref, 0.02, 2.5 );
    lin += col*0.60*dif*vec3(0.40,0.60,1.15);
    lin +=     2.00*spe*vec3(0.40,0.60,1.30)*ks;
  }
  col = lin;

  gl_FragColor = vec4(clamp(col, 0.0, 1.0), 1.0);
}`,
  },
});
