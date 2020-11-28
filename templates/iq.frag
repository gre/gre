// Created by inigo quilez - iq/2020
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.


// Basic utility functions (sdfs, noises, shaping functions)
// and also the camera setup which is shaded between the
// background rendering code ("Buffer A" tab) and the character
// rendering code ("Image" tab)


// http://iquilezles.org/www/articles/smin/smin.htm
float smin( float a, float b, float k )
{
    float h = max(k-abs(a-b),0.0);
    return min(a, b) - h*h*0.25/k;
}

// http://iquilezles.org/www/articles/smin/smin.htm
float smax( float a, float b, float k )
{
    k *= 1.4;
    float h = max(k-abs(a-b),0.0);
    return max(a, b) + h*h*h/(6.0*k*k);
}

// http://iquilezles.org/www/articles/smin/smin.htm
float smin3( float a, float b, float k )
{
    k *= 1.4;
    float h = max(k-abs(a-b),0.0);
    return min(a, b) - h*h*h/(6.0*k*k);
}

float sclamp(in float x, in float a, in float b )
{
    float k = 0.1;
	return smax(smin(x,b,k),a,k);
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float opOnion( in float sdf, in float thickness )
{
    return abs(sdf)-thickness;
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float opRepLim( in float p, in float s, in float lima, in float limb )
{
    return p-s*clamp(round(p/s),lima,limb);
}


float det( vec2 a, vec2 b ) { return a.x*b.y-b.x*a.y; }
float ndot(vec2 a, vec2 b ) { return a.x*b.x-a.y*b.y; }
float dot2( in vec2 v ) { return dot(v,v); }
float dot2( in vec3 v ) { return dot(v,v); }


// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdTorus( in vec3 p, in float ra, in float rb )
{
    return length( vec2(length(p.xz)-ra,p.y) )-rb;
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdCappedTorus(in vec3 p, in vec2 sc, in float ra, in float rb)
{
    p.x = abs(p.x);
    float k = (sc.y*p.x>sc.x*p.z) ? dot(p.xz,sc) : length(p.xz);
    return sqrt( dot(p,p) + ra*ra - 2.0*ra*k ) - rb;
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdSphere( in vec3 p, in float r )
{
    return length(p)-r;
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdEllipsoid( in vec3 p, in vec3 r )
{
    float k0 = length(p/r);
    float k1 = length(p/(r*r));
    return k0*(k0-1.0)/k1;
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdBox( in vec3 p, in vec3 b )
{
    vec3 d = abs(p) - b;
    return min( max(max(d.x,d.y),d.z),0.0) + length(max(d,0.0));
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdArc( in vec2 p, in vec2 scb, in float ra )
{
    p.x = abs(p.x);
    float k = (scb.y*p.x>scb.x*p.y) ? dot(p.xy,scb) : length(p.xy);
    return sqrt( dot(p,p) + ra*ra - 2.0*ra*k );
}

// http://research.microsoft.com/en-us/um/people/hoppe/ravg.pdf
// { dist, t, y (above the plane of the curve, x (away from curve in the plane of the curve))
vec4 sdBezier( vec3 p, vec3 va, vec3 vb, vec3 vc )
{
  vec3 w = normalize( cross( vc-vb, va-vb ) );
  vec3 u = normalize( vc-vb );
  vec3 v =          ( cross( w, u ) );
  //----
  vec2 m = vec2( dot(va-vb,u), dot(va-vb,v) );
  vec2 n = vec2( dot(vc-vb,u), dot(vc-vb,v) );
  vec3 q = vec3( dot( p-vb,u), dot( p-vb,v), dot(p-vb,w) );
  //----
  float mn = det(m,n);
  float mq = det(m,q.xy);
  float nq = det(n,q.xy);
  //----
  vec2  g = (nq+mq+mn)*n + (nq+mq-mn)*m;
  float f = (nq-mq+mn)*(nq-mq+mn) + 4.0*mq*nq;
  vec2  z = 0.5*f*vec2(-g.y,g.x)/dot(g,g);
//float t = clamp(0.5+0.5*(det(z,m+n)+mq+nq)/mn, 0.0 ,1.0 );
  float t = clamp(0.5+0.5*(det(z-q.xy,m+n))/mn, 0.0 ,1.0 );
  vec2 cp = m*(1.0-t)*(1.0-t) + n*t*t - q.xy;
  //----
  float d2 = dot(cp,cp);
  return vec4(sqrt(d2+q.z*q.z), t, q.z, -sign(f)*sqrt(d2) );
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
vec2 sdSegment(vec3 p, vec3 a, vec3 b)
{
    vec3 pa = p-a, ba = b-a;
	float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
	return vec2( length( pa - ba*h ), h );
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
vec2 sdSegmentOri(vec2 p, vec2 b)
{
	float h = clamp( dot(p,b)/dot(b,b), 0.0, 1.0 );
	return vec2( length( p - b*h ), h );
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdFakeRoundCone(vec3 p, float b, float r1, float r2)
{
    float h = clamp( p.y/b, 0.0, 1.0 );
    p.y -= b*h;
	return length(p) - mix(r1,r2,h);
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdCone( in vec3 p, in vec2 c )
{
  vec2 q = vec2( length(p.xz), p.y );

  vec2 a = q - c*clamp( (q.x*c.x+q.y*c.y)/dot(c,c), 0.0, 1.0 );
  vec2 b = q - c*vec2( clamp( q.x/c.x, 0.0, 1.0 ), 1.0 );

  float s = -sign( c.y );
  vec2 d = min( vec2( dot( a, a ), s*(q.x*c.y-q.y*c.x) ),
			    vec2( dot( b, b ), s*(q.y-c.y)  ));
  return -sqrt(d.x)*sign(d.y);
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdRhombus(vec3 p, float la, float lb, float h, float ra)
{
    p = abs(p);
    vec2 b = vec2(la,lb);
    float f = clamp( (ndot(b,b-2.0*p.xz))/dot(b,b), -1.0, 1.0 );
	vec2 q = vec2(length(p.xz-0.5*b*vec2(1.0-f,1.0+f))*sign(p.x*b.y+p.z*b.x-b.x*b.y)-ra, p.y-h);
    return min(max(q.x,q.y),0.0) + length(max(q,0.0));
}

// http://iquilezles.org/www/articles/distfunctions/distfunctions.htm
vec4 opElongate( in vec3 p, in vec3 h )
{
    vec3 q = abs(p)-h;
    return vec4( max(q,0.0), min(max(q.x,max(q.y,q.z)),0.0) );
}
//-----------------------------------------------

// ray-infinite-cylinder intersection
vec2 iCylinderY( in vec3 ro, in vec3 rd, in float rad )
{
	vec3 oc = ro;
    float a = dot( rd.xz, rd.xz );
	float b = dot( oc.xz, rd.xz );
	float c = dot( oc.xz, oc.xz ) - rad*rad;
	float h = b*b - a*c;
	if( h<0.0 ) return vec2(-1.0);
    h = sqrt(h);
	return vec2(-b-h,-b+h)/a;
}

// ray-infinite-cone intersection
vec2 iConeY(in vec3 ro, in vec3 rd, in float k )
{
	float a = dot(rd.xz,rd.xz) - k*rd.y*rd.y;
    float b = dot(ro.xz,rd.xz) - k*ro.y*rd.y;
    float c = dot(ro.xz,ro.xz) - k*ro.y*ro.y;

    float h = b*b-a*c;
    if( h<0.0 ) return vec2(-1.0);
    h = sqrt(h);
    return vec2(-b-h,-b+h)/a;
}

//-----------------------------------------------

float linearstep(float a, float b, in float x )
{
    return clamp( (x-a)/(b-a), 0.0, 1.0 );
}

vec2 rot( in vec2 p, in float an )
{
    float cc = cos(an);
    float ss = sin(an);
    return mat2(cc,-ss,ss,cc)*p;
}

float expSustainedImpulse( float t, float f, float k )
{
    return smoothstep(0.0,f,t)*1.1 - 0.1*exp2(-k*max(t-f,0.0));
}

//-----------------------------------------------

vec3 hash3( uint n )
{
    // integer hash copied from Hugo Elias
	n = (n << 13U) ^ n;
    n = n * (n * n * 15731U + 789221U) + 1376312589U;
    uvec3 k = n * uvec3(n,n*16807U,n*48271U);
    return vec3( k & uvec3(0x7fffffffU))/float(0x7fffffff);
}

//---------------------------------------

float noise1( sampler3D tex, in vec3 x )
{
    return textureLod(tex,(x+0.5)/32.0,0.0).x;
}
float noise1( sampler2D tex, in vec2 x )
{
    return textureLod(tex,(x+0.5)/64.0,0.0).x;
}
float noise1f( sampler2D tex, in vec2 x )
{
    return texture(tex,(x+0.5)/64.0).x;
}
float fbm1( sampler3D tex, in vec3 x )
{
    float f = 0.0;
    f += 0.5000*noise1(tex,x); x*=2.01;
    f += 0.2500*noise1(tex,x); x*=2.01;
    f += 0.1250*noise1(tex,x); x*=2.01;
    f += 0.0625*noise1(tex,x);
    f = 2.0*f-0.9375;
    return f;
}

float fbm1( sampler2D tex, in vec2 x )
{
    float f = 0.0;
    f += 0.5000*noise1(tex,x); x*=2.01;
    f += 0.2500*noise1(tex,x); x*=2.01;
    f += 0.1250*noise1(tex,x); x*=2.01;
    f += 0.0625*noise1(tex,x);
    f = 2.0*f-0.9375;
    return f;
}
float fbm1f( sampler2D tex, in vec2 x )
{
    float f = 0.0;
    f += 0.5000*noise1f(tex,x); x*=2.01;
    f += 0.2500*noise1f(tex,x); x*=2.01;
    f += 0.1250*noise1f(tex,x); x*=2.01;
    f += 0.0625*noise1f(tex,x);
    f = 2.0*f-0.9375;
    return f;
}
float bnoise( in float x )
{
    float i = floor(x);
    float f = fract(x);
    float s = sign(fract(x/2.0)-0.5);
    float k = 0.5+0.5*sin(i);
    return s*f*(f-1.0)*((16.0*k-4.0)*f*(f-1.0)-1.0);
}

vec3 fbm13( in float x, in float g )
{
    vec3 n = vec3(0.0);
    float s = 1.0;
    for( int i=0; i<6; i++ )
    {
        n += s*vec3(bnoise(x),bnoise(x+13.314),bnoise(x+31.7211));
        s *= g;
        x *= 2.01;
        x += 0.131;
    }
    return n;
}


//--------------------------------------------------
//const float X1 = 1.6180339887498948; const float H1 = float( 1.0/X1 );
//const float X2 = 1.3247179572447460; const vec2  H2 = vec2(  1.0/X2, 1.0/(X2*X2) );
//const float X3 = 1.2207440846057595; const vec3  H3 = vec3(  1.0/X3, 1.0/(X3*X3), 1.0/(X3*X3*X3) );
  const float X4 = 1.1673039782614187; const vec4  H4 = vec4(  1.0/X4, 1.0/(X4*X4), 1.0/(X4*X4*X4), 1.0/(X4*X4*X4*X4) );

//--------------------------------------
mat3 calcCamera( in float time, out vec3 oRo, out float oFl )
{
    vec3 ta = vec3( 0.0, -0.3, 0.0 );
    vec3 ro = vec3( -0.5563, -0.2, 2.7442 );
    float fl = 1.7;
#if 0
    vec3 fb = fbm13( 0.2*time, 0.5 );
    ta += 0.025*fb;
    float cr = -0.01 + 0.006*fb.z;
#else
    vec3 fb1 = fbm13( 0.15*time, 0.50 );
    ro.xyz += 0.010*fb1.xyz;
    vec3 fb2 = fbm13( 0.33*time, 0.65 );
    fb2 = fb2*fb2*sign(fb2);
    ta.xy += 0.005*fb2.xy;
    float cr = -0.01 + 0.002*fb2.z;
#endif

    // camera matrix
    vec3 ww = normalize( ta - ro );
    vec3 uu = normalize( cross(ww,vec3(sin(cr),cos(cr),0.0) ) );
    vec3 vv =          ( cross(uu,ww));

    oRo = ro;
    oFl = fl;

    return mat3(uu,vv,ww);
}

#define ZERO min(iFrame,0)
#define ZEROU min(uint(iFrame),0u)
