(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[4037],{4273:function(e,t,n){"use strict";n.r(t);n(7294);var r=n(3224),o=n(7250),i=n(5893);const c=()=>(0,i.jsx)(r.Node,{shader:a.node,uniforms:{resolution:r.Uniform.Resolution,image:"/images/shaders/binance.dist.png"}}),a=r.Shaders.create({node:{frag:r.GLSL`
precision highp float;
varying vec2 uv;
uniform vec2 resolution;
uniform sampler2D image;
#define PI ${Math.PI}

// from https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
float hash(float n) { return fract(sin(n) * 1e4); }
float hash(vec2 p) { return fract(1e4 * sin(17.0 * p.x + p.y * 0.1) * (0.1 + abs(sin(p.y * 13.0 + p.x)))); }
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
    vec3(0.5, 0.5, 0.3),
    vec3(0.5, 0.5, 0.5),
    vec3(0.9, 0.8, 1.0),
    vec3(0.4, 0.2, 1.0)
  );
}

float scene(in vec2 p) {
  vec2 q = vec2( fbm( 2. * p + vec2(4.2, 1.8) ), fbm( 10. * p ) );
  vec2 r = vec2( fbm(10.0 * q),
                fbm( 80. * q));
  float v = 0.5 * fbm(p + 0.3 * r + 3.0);
  v += 0.5 * texture2D(image, uv).a + 0.1;
  return .5 * fbm(q) + smoothstep(0.3, 1.0, fract(v));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec3 c = vec3(0.);
  vec2 p = (uv - 0.5) * ratio;
  c += pal(scene(p));
  gl_FragColor = vec4(c, 1.0);
}
`}});t.default=function(){return(0,i.jsx)(o.T,{width:500,height:500,children:(0,i.jsx)(c,{})})}},1660:function(e,t,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/playground/ethwarp01",function(){return n(4273)}])},4155:function(e){var t,n,r=e.exports={};function o(){throw new Error("setTimeout has not been defined")}function i(){throw new Error("clearTimeout has not been defined")}function c(e){if(t===setTimeout)return setTimeout(e,0);if((t===o||!t)&&setTimeout)return t=setTimeout,setTimeout(e,0);try{return t(e,0)}catch(n){try{return t.call(null,e,0)}catch(n){return t.call(this,e,0)}}}!function(){try{t="function"===typeof setTimeout?setTimeout:o}catch(e){t=o}try{n="function"===typeof clearTimeout?clearTimeout:i}catch(e){n=i}}();var a,u=[],f=!1,s=-1;function l(){f&&a&&(f=!1,a.length?u=a.concat(u):s=-1,u.length&&h())}function h(){if(!f){var e=c(l);f=!0;for(var t=u.length;t;){for(a=u,u=[];++s<t;)a&&a[s].run();s=-1,t=u.length}a=null,f=!1,function(e){if(n===clearTimeout)return clearTimeout(e);if((n===i||!n)&&clearTimeout)return n=clearTimeout,clearTimeout(e);try{n(e)}catch(t){try{return n.call(null,e)}catch(t){return n.call(this,e)}}}(e)}}function v(e,t){this.fun=e,this.array=t}function m(){}r.nextTick=function(e){var t=new Array(arguments.length-1);if(arguments.length>1)for(var n=1;n<arguments.length;n++)t[n-1]=arguments[n];u.push(new v(e,t)),1!==u.length||f||c(h)},v.prototype.run=function(){this.fun.apply(null,this.array)},r.title="browser",r.browser=!0,r.env={},r.argv=[],r.version="",r.versions={},r.on=m,r.addListener=m,r.once=m,r.off=m,r.removeListener=m,r.removeAllListeners=m,r.emit=m,r.prependListener=m,r.prependOnceListener=m,r.listeners=function(e){return[]},r.binding=function(e){throw new Error("process.binding is not supported")},r.cwd=function(){return"/"},r.chdir=function(e){throw new Error("process.chdir is not supported")},r.umask=function(){return 0}}},function(e){e.O(0,[9774,8764,7250,2888,179],(function(){return t=1660,e(e.s=t);var t}));var t=e.O();_N_E=t}]);