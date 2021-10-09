(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[151],{3505:function(e,t,o){"use strict";o.r(t);var n=o(5893),r=o(7294),a=o(3224),i=o(7250);const f=[.5+Math.random()*(.5-Math.random()),.4+.5*Math.random()*Math.random(),.5+Math.random()*(.5-Math.random())],c=100*Math.random(),s=100*Math.random(),l=100*Math.random(),u=()=>{const e=function(){const{0:e,1:t}=(0,r.useState)(0);return(0,r.useEffect)((()=>{let e,o;return o=requestAnimationFrame((function n(r){o=requestAnimationFrame(n),e||(e=r),t((r-e)/1e3)})),()=>cancelAnimationFrame(o)}),[]),e}();return(0,n.jsx)(a.Node,{shader:p.node,uniforms:{resolution:a.Uniform.Resolution,time:e,colordelta:f,s1:c,s2:s,s3:l}})},p=a.Shaders.create({node:{frag:a.GLSL`
precision highp float;
varying vec2 uv;
uniform vec2 resolution;
uniform float time;
uniform float s1, s2, s3;
uniform vec3 colordelta;
#define PI ${Math.PI}


// Maximum/minumum elements of a vector
float vmax(vec2 v) {
	return max(v.x, v.y);
}

float vmax(vec3 v) {
	return max(max(v.x, v.y), v.z);
}

float vmax(vec4 v) {
	return max(max(v.x, v.y), max(v.z, v.w));
}

float vmin(vec2 v) {
	return min(v.x, v.y);
}

float vmin(vec3 v) {
	return min(min(v.x, v.y), v.z);
}

float vmin(vec4 v) {
	return min(min(v.x, v.y), min(v.z, v.w));
}
float fBox2(vec2 p, vec2 b) {
	vec2 d = abs(p) - b;
	return length(max(d, vec2(0.))) + vmax(min(d, vec2(0.)));
}
float fOpUnionRound(float a, float b, float r) {
	vec2 u = max(vec2(r - a,r - b), vec2(0.));
	return max(r, min (a, b)) - length(u);
}
float fOpIntersectionRound(float a, float b, float r) {
	vec2 u = max(vec2(r + a,r + b), vec2(0));
	return min(-r, max (a, b)) + length(u);
}
float fOpDifferenceRound (float a, float b, float r) {
	return fOpIntersectionRound(a, -b, r);
}
// Repeat space along one axis. Use like this to repeat along the x axis:
// <float cell = pMod1(p.x,5);> - using the return value is optional.
float pMod1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize, size) - halfsize;
	return c;
}

// Same, but mirror every second cell so they match at the boundaries
float pModMirror1(inout float p, float size) {
	float halfsize = size*0.5;
	float c = floor((p + halfsize)/size);
	p = mod(p + halfsize,size) - halfsize;
	p *= mod(c, 2.0)*2. - 1.;
	return c;
}
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
const mat2 m2 = mat2( 0.6,  0.8, -0.8,  0.6 );
float fbm( in vec2 x ) {
  float f = 2.0;
  float s = 0.55;
  float a = 0.0;
  float b = 0.5;
  for( int i=0; i<12; i++ ) {
    float n = noise(x);
    a += b * n;
    b *= s;
    x = f * x;
  }
	return a;
}

vec3 palette(float t,vec3 a,vec3 b,vec3 c,vec3 d){
  return a+b*cos(6.28318*(c*t+d));
}
vec3 pal(float t){
  return palette(
    t,
    vec3(0.5, 0.5, 0.5),
    colordelta,
    vec3(1.0),
    vec3(0.49, 0.5, 0.51)
  );
}

vec3 scene(in vec2 p) {
  vec2 q = vec2( fbm( p + vec2(4.2, 1.8) ), fbm( 2. * p ) );
  vec2 r = vec2( fbm(7. * q),
                fbm(s1 + p + 80. * q));
  float v = fbm(0.8 * p + .2 * r + s2);

  vec2 o = p;

  p.x = abs(p.x);
  // pModMirror1(p.x, 0.4);
  // pModMirror1(p.y, 0.4);

  // v -= p.x * 4.;
  // v += p.y * 10.;

  float s = fbm(
    4.0 * p +
    s3 +
    fbm(vec2(fbm(3.4 * p + s1), fbm(4. * p + s2)))
  ) + length(p) - 0.7 + 0.01 * cos(6. * time);
  float diff = length(p - vec2(0.15, 0.0))-0.05;

  // s = min(s, 0.7 * p.x + 0.3 * abs(p.y) - 0.1);
  // s = min(s, p.x);

  float k = 0.15;

  p.x -= 0.05;
  p.y -= 0.1 * cos(1.3 * time);
  pR(p, s1 + 0.2 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.1)), k);
  p.x += 0.2;
  p.y += 0.1;
  pR(p, 1.0 + 0.2 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.2)), k);
  p.x -= 0.1 * sin(0.3 * time);
  p.y -= 0.1 * cos(time);
  pR(p, 2.0 - 0.4 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.2)), k);
  p.x += 0.1 * cos(.3 * time);
  p.y += 0.1 * sin(.2 * time);
  pR(p, 0.4 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.03, 0.1)), k);
 
  /*
  p = o;
  pR(p, 0.3 * time);
  s = fOpUnionRound(s, fBox2(p, vec2(0.1, 0.4)), k);
  */

  s = fOpDifferenceRound(s, diff, k);

  float d = smoothstep(0.0, 0.2, s);
  v = mix(v, 0.5, smoothstep(0.01, 0.0, s));
  v -= .5 * pow(d, 0.2);

  return pal(v) + smoothstep(0.0, -0.2, s);
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  c += scene(p);
  gl_FragColor = vec4(c, 1.0);
}
`}});t.default=function(){return(0,n.jsx)(i.T,{pixelRatio:1,width:500,height:500,children:(0,n.jsx)(u,{})})}},2103:function(e,t,o){(window.__NEXT_P=window.__NEXT_P||[]).push(["/playground/wip",function(){return o(3505)}])},4155:function(e){var t,o,n=e.exports={};function r(){throw new Error("setTimeout has not been defined")}function a(){throw new Error("clearTimeout has not been defined")}function i(e){if(t===setTimeout)return setTimeout(e,0);if((t===r||!t)&&setTimeout)return t=setTimeout,setTimeout(e,0);try{return t(e,0)}catch(o){try{return t.call(null,e,0)}catch(o){return t.call(this,e,0)}}}!function(){try{t="function"===typeof setTimeout?setTimeout:r}catch(e){t=r}try{o="function"===typeof clearTimeout?clearTimeout:a}catch(e){o=a}}();var f,c=[],s=!1,l=-1;function u(){s&&f&&(s=!1,f.length?c=f.concat(c):l=-1,c.length&&p())}function p(){if(!s){var e=i(u);s=!0;for(var t=c.length;t;){for(f=c,c=[];++l<t;)f&&f[l].run();l=-1,t=c.length}f=null,s=!1,function(e){if(o===clearTimeout)return clearTimeout(e);if((o===a||!o)&&clearTimeout)return o=clearTimeout,clearTimeout(e);try{o(e)}catch(t){try{return o.call(null,e)}catch(t){return o.call(this,e)}}}(e)}}function v(e,t){this.fun=e,this.array=t}function m(){}n.nextTick=function(e){var t=new Array(arguments.length-1);if(arguments.length>1)for(var o=1;o<arguments.length;o++)t[o-1]=arguments[o];c.push(new v(e,t)),1!==c.length||s||i(p)},v.prototype.run=function(){this.fun.apply(null,this.array)},n.title="browser",n.browser=!0,n.env={},n.argv=[],n.version="",n.versions={},n.on=m,n.addListener=m,n.once=m,n.off=m,n.removeListener=m,n.removeAllListeners=m,n.emit=m,n.prependListener=m,n.prependOnceListener=m,n.listeners=function(e){return[]},n.binding=function(e){throw new Error("process.binding is not supported")},n.cwd=function(){return"/"},n.chdir=function(e){throw new Error("process.chdir is not supported")},n.umask=function(){return 0}}},function(e){e.O(0,[774,976,250,888,179],(function(){return t=2103,e(e.s=t);var t}));var t=e.O();_N_E=t}]);