(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[1175],{2266:function(e,o,n){"use strict";n.d(o,{h:function(){return s}});var t=n(5988),i=(n(7294),n(5893));function s({children:e}){return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)("header",{className:"jsx-2180230042",children:e}),(0,i.jsx)(t.default,{id:"2180230042",children:["header.jsx-2180230042{margin:1rem 0;display:-webkit-box;display:-webkit-flex;display:-ms-flexbox;display:flex;-webkit-flex-direction:column;-ms-flex-direction:column;flex-direction:column;-webkit-align-items:center;-webkit-box-align:center;-ms-flex-align:center;align-items:center;font-size:16px;}"]})]})}},3137:function(e,o,n){"use strict";n.r(o),n.d(o,{default:function(){return y}});var t=n(7294),i=n(9008),s=n(9701),r=n(1532),a=n(4276),l=n(3369),c=n(2266),d=n(7250),f=n(9296),u=n(3224),m=n(7996),h=n.n(m),v=n(9920),p=n(5893);const x=u.Shaders.create({main:{frag:u.GLSL`
precision highp float;
varying vec2 uv;

uniform vec2 resolution;
uniform float mod2, mod1, mod3;
uniform float s1, s2, s3, s4, s5, s6, s7, s8, s9;
uniform float rotation;

const float PI = ${Math.PI};
void pR(inout vec2 p, float a) {
	p = cos(a)*p + sin(a)*vec2(p.y, -p.x);
}
vec3 palette( in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d ) {
  return a + b*cos( 6.28318*(c*t+d) );
}
vec3 pal (float t) {
  return palette(
    t + 0.5 * mod3 * mod3,
    vec3(.85 - .5 * mod3),
    vec3(.5),
    vec3(1.),
    vec3(0.8 + 0.2 * s1, 0.2 * s2, .2)
  );
}
float run (vec2 init) {
  float iterations = 2. + 500. * pow(mod2, 2.0);
  vec2 p = init;
  for (float iter = 0.; iter < 502.; iter += 1.) {
    if (iter >= iterations) break;
    // original mandelbrot formula is: p = vec2(p.x * p.x - p.y * p.y, 2. * p.x * p.y) + init;
    float x2 = p.x * p.x;
    float y2 = p.y * p.y;
    float xy = p.x * p.y;
    float a = 1. + .1 * (s1 - 0.5) * s2 * s2;
    float b = -1. + .1 * (s1 - 0.5) * s2 * s2;
    float c = 0.0 + 2. * (s2 - 0.5) * s3 * s3;
    float d = max(0., pow(s8, 5.) - 0.5) * cos(100. * s7 * s2 * s9 * p.y);
    float e = max(0., pow(s9, 5.) - 0.5) * sin(100. * s2 * s1 * p.x);
    float f = 2. + s6 - s6 * s6 * s6;
    vec2 offset = init + mix(vec2(0.0), vec2(s4, s5) - .5, s3 * s4 * s5);
    p = vec2(
      a * x2 + b * y2 + c * xy + d,
      f * xy + e
    ) + offset;
    if (length(p) >= 2.0) {
      return iter / iterations;
    }
  }
  return 0.;
}
vec3 shade (vec2 uv) {
  float zoom = (0.3 + 12. * s7 * s7 * s7) * (1. + 3. * mod1);
  float focusAngle = 4. * mod1;
  float focusAmp = 0.4 * s7;
  vec2 init = 2. * (uv - .5) / zoom;
  pR(init, rotation);
  init -= vec2(.8, .0);
  init += focusAmp * vec2(cos(focusAngle), sin(focusAngle));
  return pal(pow(run(init), .5));
}

void main() {
  vec2 ratio = resolution / min(resolution.x, resolution.y);
  vec2 uvRatio = 0.5 + (uv - 0.5) * ratio;
  vec3 c = shade(uvRatio);
  gl_FragColor = vec4(c, 1.0);
}
  `}}),g=(0,v.animated)((({values:[e,o,n,t,i,s,r,a,l,c,d,f,m]})=>(0,p.jsx)(u.Node,{shader:x.main,uniforms:{resolution:u.Uniform.Resolution,mod2:o,mod1:e,mod3:n,s1:t,s2:i,s3:s,s4:r,s5:a,s6:l,s7:c,s8:d,s9:f,rotation:m}}))),w=({mint:e})=>{const o=(0,t.useMemo)((()=>function({block:e,mod1:o,mod2:n,mod3:t}){const{hash:i}=e,s=new(h())(parseInt(i.slice(0,16),16)),r=s.random(),a=s.random(),l=s.random(),c=s.random();return[o,n,t,r,a,l,c,c,s.random(),s.random(),s.random(),s.random(),s.random(),Math.PI*Math.floor(.5+8*l)/4]}(e)),[e]),{values:n}=(0,v.useSpring)({values:o,config:{mass:1,tension:50,friction:30}});return(0,p.jsx)(g,{values:n})};function y(){const{observe:e,width:o,height:n}=(0,s.ZP)({}),u=(0,f.useRandomBlocks)(),m=(0,t.useMemo)((()=>u.map((e=>({block:e,mod1:Math.random(),mod2:Math.random(),mod3:Math.random()})))),[u]),{0:h,1:v}=(0,t.useState)(0);(0,t.useEffect)((()=>{const e=setInterval((()=>{v((e=>e+1))}),3e3);return()=>clearInterval(e)}),[3e3]);const x=m[h%m.length];return x?(0,p.jsx)(a.x,{children:(0,p.jsxs)(r.W,{children:[(0,p.jsxs)(i.default,{children:[(0,p.jsx)("title",{children:"Slideshow mandelglitch"}),(0,p.jsx)("link",{rel:"icon",href:"/favicon.ico"})]}),(0,p.jsxs)(l.o,{children:[(0,p.jsx)(c.h,{children:(0,p.jsx)("h1",{children:"Mandelglitch slideshow experiment"})}),(0,p.jsx)("div",{style:{display:"flex",flexDirection:"row"},children:(0,p.jsx)("div",{ref:e,style:{width:"60vw",height:"60vw"},children:(0,p.jsx)(d.T,{width:o,height:n,children:(0,p.jsx)(w,{mint:x})})})})]})]})}):null}},8971:function(e,o,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/playground/slideshow-mandelglitch",function(){return n(3137)}])}},function(e){e.O(0,[9774,5988,8764,7250,1900,5050,9296,2888,179],(function(){return o=8971,e(e.s=o);var o}));var o=e.O();_N_E=o}]);