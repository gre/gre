(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[1175],{2266:function(e,o,n){"use strict";n.d(o,{h:function(){return s}});var t=n(5893),i=n(5988);n(7294);function s({children:e}){return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)("header",{className:"jsx-2180230042",children:e}),(0,t.jsx)(i.default,{id:"2180230042",children:["header.jsx-2180230042{margin:1rem 0;display:-webkit-box;display:-webkit-flex;display:-ms-flexbox;display:flex;-webkit-flex-direction:column;-ms-flex-direction:column;flex-direction:column;-webkit-align-items:center;-webkit-box-align:center;-ms-flex-align:center;align-items:center;font-size:16px;}"]})]})}},3137:function(e,o,n){"use strict";n.r(o),n.d(o,{default:function(){return y}});var t=n(5893),i=n(7294),s=n(9008),r=n(9701),a=n(1532),l=n(4276),c=n(3369),d=n(2266),f=n(7250),u=n(9296),m=n(3224),h=n(7996),v=n.n(h),p=n(9920);const x=m.Shaders.create({main:{frag:m.GLSL`
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
  `}}),g=(0,p.animated)((({values:[e,o,n,i,s,r,a,l,c,d,f,u,h]})=>(0,t.jsx)(m.Node,{shader:x.main,uniforms:{resolution:m.Uniform.Resolution,mod2:o,mod1:e,mod3:n,s1:i,s2:s,s3:r,s4:a,s5:l,s6:c,s7:d,s8:f,s9:u,rotation:h}}))),w=({mint:e})=>{const o=(0,i.useMemo)((()=>function({block:e,mod1:o,mod2:n,mod3:t}){const{hash:i}=e,s=new(v())(parseInt(i.slice(0,16),16)),r=s.random(),a=s.random(),l=s.random(),c=s.random();return[o,n,t,r,a,l,c,c,s.random(),s.random(),s.random(),s.random(),s.random(),Math.PI*Math.floor(.5+8*l)/4]}(e)),[e]),{values:n}=(0,p.useSpring)({values:o,config:{mass:1,tension:50,friction:30}});return(0,t.jsx)(g,{values:n})};function y(){const{observe:e,width:o,height:n}=(0,r.ZP)({}),m=(0,u.useRandomBlocks)(),h=(0,i.useMemo)((()=>m.map((e=>({block:e,mod1:Math.random(),mod2:Math.random(),mod3:Math.random()})))),[m]),{0:v,1:p}=(0,i.useState)(0);(0,i.useEffect)((()=>{const e=setInterval((()=>{p((e=>e+1))}),3e3);return()=>clearInterval(e)}),[3e3]);const x=h[v%h.length];return x?(0,t.jsx)(l.x,{children:(0,t.jsxs)(a.W,{children:[(0,t.jsxs)(s.default,{children:[(0,t.jsx)("title",{children:"Slideshow mandelglitch"}),(0,t.jsx)("link",{rel:"icon",href:"/favicon.ico"})]}),(0,t.jsxs)(c.o,{children:[(0,t.jsx)(d.h,{children:(0,t.jsx)("h1",{children:"Mandelglitch slideshow experiment"})}),(0,t.jsx)("div",{style:{display:"flex",flexDirection:"row"},children:(0,t.jsx)("div",{ref:e,style:{width:"60vw",height:"60vw"},children:(0,t.jsx)(f.T,{width:o,height:n,children:(0,t.jsx)(w,{mint:x})})})})]})]})}):null}},8971:function(e,o,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/playground/slideshow-mandelglitch",function(){return n(3137)}])}},function(e){e.O(0,[9774,5988,8764,7250,5822,5050,9296,2888,179],(function(){return o=8971,e(e.s=o);var o}));var o=e.O();_N_E=o}]);