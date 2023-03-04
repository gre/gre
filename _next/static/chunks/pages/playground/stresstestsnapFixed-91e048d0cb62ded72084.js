(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[1726],{1532:function(e,t,n){"use strict";n.d(t,{W:function(){return o}});var r=n(5988),i=(n(7294),n(5893));function o({children:e}){return(0,i.jsxs)("div",{className:"jsx-3621368397 container",children:[e,(0,i.jsx)(r.default,{id:"3621368397",children:[".container.jsx-3621368397{min-height:100vh;padding:0 0.5rem;display:-webkit-box;display:-webkit-flex;display:-ms-flexbox;display:flex;-webkit-flex-direction:column;-ms-flex-direction:column;flex-direction:column;-webkit-box-pack:center;-webkit-justify-content:center;-ms-flex-pack:center;justify-content:center;-webkit-align-items:center;-webkit-box-align:center;-ms-flex-align:center;align-items:center;}"]})]})}},4276:function(e,t,n){"use strict";n.d(t,{x:function(){return c},M:function(){return s}});var r=n(5988),i=(n(7294),n(6189)),o=n(5893);function c({children:e}){return(0,o.jsxs)(i.Z,{children:[e,(0,o.jsx)(r.default,{id:"3469673304",children:["html,body{padding:0;margin:0;font-family:-apple-system,BlinkMacSystemFont,Segoe UI,Roboto, Oxygen,Ubuntu,Cantarell,Fira Sans,Droid Sans,Helvetica Neue, sans-serif;}","*{box-sizing:border-box;}","a{color:inherit;-webkit-text-decoration:none;text-decoration:none;}","a:hover,a:active{-webkit-text-decoration:underline;text-decoration:underline;}"]})]})}function s({children:e}){return(0,o.jsxs)(c,{children:[e,(0,o.jsx)(r.default,{id:"2550269578",children:["body{background:#000;color:#fff;}"]})]})}},2266:function(e,t,n){"use strict";n.d(t,{h:function(){return o}});var r=n(5988),i=(n(7294),n(5893));function o({children:e}){return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)("header",{className:"jsx-2180230042",children:e}),(0,i.jsx)(r.default,{id:"2180230042",children:["header.jsx-2180230042{margin:1rem 0;display:-webkit-box;display:-webkit-flex;display:-ms-flexbox;display:flex;-webkit-flex-direction:column;-ms-flex-direction:column;flex-direction:column;-webkit-align-items:center;-webkit-box-align:center;-ms-flex-align:center;align-items:center;font-size:16px;}"]})]})}},3369:function(e,t,n){"use strict";n.d(t,{o:function(){return o}});var r=n(5988),i=(n(7294),n(5893));function o({children:e}){return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)("main",{className:"jsx-1508801263",children:e}),(0,i.jsx)(r.default,{id:"1508801263",children:["main.jsx-1508801263{-webkit-flex:1;-ms-flex:1;flex:1;display:-webkit-box;display:-webkit-flex;display:-ms-flexbox;display:flex;-webkit-flex-direction:column;-ms-flex-direction:column;flex-direction:column;-webkit-box-pack:center;-webkit-justify-content:center;-ms-flex-pack:center;justify-content:center;-webkit-align-items:center;-webkit-box-align:center;-ms-flex-align:center;align-items:center;}"]})]})}},4835:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return m}});var r=n(7294),i=n(3224),o=n(7250),c=n(9008),s=n(9701),a=n(1532),l=n(4276),u=n(3369),f=n(2266),d=n(5893);const h=e=>new Promise((t=>setTimeout(t,e))),p=i.Shaders.create({main:{frag:i.GLSL`
precision highp float;
varying vec2 uv;
uniform vec2 resolution;
uniform float time;
#define PI ${Math.PI}

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
    t + time,
    vec3(0.8, 0.5, 0.4),
    vec3(0.5, 0.4, 0.2),
    vec3(1.1),
    vec3(0.9, 0.7, 0.5)
    );
}

float scene(in vec2 p) {
    float t = 0.2 * PI * time;
    vec2 q = vec2( fbm( 5. * p + vec2(4.2, 1.8) ), fbm( 5. * p ) );
    vec2 r = vec2( fbm(30.1 * q + 6.0 * vec2(cos(t), sin(t))),
                fbm( 50. * q) );
    float v = 0.4 * fbm(p + 3. * r + 10.);
    v += 0.6 * pow(fbm(2. * r + p + vec2(cos(t), sin(t))), 3.0);
    return v;
}

void main() {
    vec2 ratio = resolution / min(resolution.x, resolution.y);
    vec3 c = vec3(0.);
    vec2 p = (uv - 0.5) * ratio;
    c += pal(scene(p));
    gl_FragColor = vec4(c, 1.0);
}
    `}});const x=({width:e,height:t,surfaceRef:n})=>{const c=function(){const{0:e,1:t}=(0,r.useState)(0);return(0,r.useEffect)((()=>{let e,n;return n=requestAnimationFrame((function r(i){n=requestAnimationFrame(r),e||(e=i),t((i-e)/1e3)})),()=>cancelAnimationFrame(n)}),[]),e}();return(0,d.jsx)(o.T,{webglContextAttributes:{preserveDrawingBuffer:!0},width:e,height:t,ref:n,children:(0,d.jsx)(i.Node,{shader:p.main,uniforms:{time:c,resolution:i.Uniform.Resolution}})})},b=({burn:e})=>{const t=Date.now();for(;Date.now()-t<e;);return null};function m(){const{observe:e,width:t,height:n}=(0,s.ZP)({}),i=(0,r.useRef)(),{0:o,1:p}=(0,r.useState)(0),{0:m,1:v}=(0,r.useState)(!1),{0:w,1:y}=(0,r.useState)(),g=(0,r.useRef)();return(0,r.useLayoutEffect)((()=>{m&&(g.current||(g.current=!0,async function(){try{if(await h(1e3),!i.current)return console.log("no ref?"),void v(!1);const e=await i.current.captureAsBlob(),t=URL.createObjectURL(e);await h(300),console.log("ok",t),y({img:e,url:t}),v(!1)}catch(e){console.error("ko",e),v(!1)}}().then((()=>{g.current=!1}))))}),[m,i]),(0,r.useEffect)((()=>{o<=0||v(!0)}),[o]),(0,d.jsx)(l.x,{children:(0,d.jsxs)(a.W,{children:[(0,d.jsxs)(c.default,{children:[(0,d.jsx)("title",{children:"stress test"}),(0,d.jsx)("link",{rel:"icon",href:"/favicon.ico"})]}),(0,d.jsxs)(u.o,{children:[(0,d.jsx)(b,{burn:950}),(0,d.jsx)(f.h,{children:(0,d.jsx)("h1",{children:"Mandelglitch slideshow experiment"})}),(0,d.jsx)("button",{onClick:()=>p((e=>e+1)),children:"SNAP"}),w?(0,d.jsx)("div",{style:{border:"4px solid red"},children:(0,d.jsx)("img",{src:w.url,width:100})}):null,(0,d.jsxs)("div",{style:{display:"flex",flexDirection:"row"},children:[(0,d.jsx)("div",{ref:e,style:{width:m?"80vw":"20vw",height:m?"80vw":"20vw"},children:(0,d.jsx)(x,{surfaceRef:i,width:t,height:n})}),(0,d.jsx)(b,{burn:100})]})]})]})})}},8867:function(e,t,n){(window.__NEXT_P=window.__NEXT_P||[]).push(["/playground/stresstestsnapFixed",function(){return n(4835)}])},9008:function(e,t,n){e.exports=n(639)},9701:function(e,t,n){"use strict";var r=n(7294);function i(){return(i=Object.assign||function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var r in n)Object.prototype.hasOwnProperty.call(n,r)&&(e[r]=n[r])}return e}).apply(this,arguments)}var o=function(e){var t=(0,r.useRef)(e);return t.current=e,t};t.ZP=function(e){var t=void 0===e?{}:e,n=t.useBorderBoxSize,c=t.breakpoints,s=t.updateOnBreakpointChange,a=t.shouldUpdate,l=t.onResize,u=t.polyfill,f=(0,r.useState)({currentBreakpoint:"",width:0,height:0}),d=f[0],h=f[1],p=(0,r.useRef)({}),x=(0,r.useRef)(),b=(0,r.useRef)(),m=(0,r.useRef)(!1),v=(0,r.useRef)(),w=o(l),y=o(a),g=(0,r.useCallback)((function(){b.current&&b.current.disconnect()}),[]),k=(0,r.useCallback)((function(e){e&&e!==v.current&&(g(),v.current=e),b.current&&v.current&&b.current.observe(v.current)}),[g]);return(0,r.useEffect)((function(){if((!("ResizeObserver"in window)||!("ResizeObserverEntry"in window))&&!u)return console.error("\ud83d\udca1 react-cool-dimensions: the browser doesn't support Resize Observer, please use polyfill: https://github.com/wellyshen/react-cool-dimensions#resizeobserver-polyfill"),function(){return null};var e=null;return b.current=new(u||ResizeObserver)((function(t){var r=t[0];e=requestAnimationFrame((function(){var e=r.contentBoxSize,t=r.borderBoxSize,i=r.contentRect,o=e;n&&(t?o=t:m.current||(console.warn("\ud83d\udca1 react-cool-dimensions: the browser doesn't support border-box size, fallback to content-box size. Please see: https://github.com/wellyshen/react-cool-dimensions#border-box-size-measurement"),m.current=!0));var a=(o=Array.isArray(o)?o[0]:o)?o.inlineSize:i.width,l=o?o.blockSize:i.height;if(a!==p.current.width||l!==p.current.height){p.current={width:a,height:l};var u={currentBreakpoint:"",width:a,height:l,entry:r,observe:k,unobserve:g};c?(u.currentBreakpoint=function(e,t){var n="",r=-1;return Object.keys(e).forEach((function(i){var o=e[i];t>=o&&o>r&&(n=i,r=o)})),n}(c,a),u.currentBreakpoint!==x.current&&(w.current&&w.current(u),x.current=u.currentBreakpoint)):w.current&&w.current(u);var f={currentBreakpoint:u.currentBreakpoint,width:a,height:l,entry:r};y.current&&!y.current(f)||(!y.current&&c&&s?h((function(e){return e.currentBreakpoint!==f.currentBreakpoint?f:e})):h(f))}}))})),k(),function(){g(),e&&cancelAnimationFrame(e)}}),[JSON.stringify(c),n,k,g,s]),i({},d,{observe:k,unobserve:g})}},6189:function(e,t,n){"use strict";var r,i=n(6042),o=(r=i)&&r.__esModule?r:{default:r};var c=function(e){var t,n=e.disabled,r=e.children;return n||"undefined"===typeof window||!window.location||"http:"!==window.location.protocol||("localhost"===(t=window.location.hostname)||"[::1]"===t||t.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/))?r:(window.location.href=window.location.href.replace(/^http(?!s)/,"https"),null)};c.propTypes={children:o.default.node,disabled:o.default.bool},t.Z=c},6747:function(e,t,n){"use strict";var r=n(6438);function i(){}function o(){}o.resetWarningCache=i,e.exports=function(){function e(e,t,n,i,o,c){if(c!==r){var s=new Error("Calling PropTypes validators directly is not supported by the `prop-types` package. Use PropTypes.checkPropTypes() to call them. Read more at http://fb.me/use-check-prop-types");throw s.name="Invariant Violation",s}}function t(){return e}e.isRequired=e;var n={array:e,bigint:e,bool:e,func:e,number:e,object:e,string:e,symbol:e,any:e,arrayOf:t,element:e,elementType:e,instanceOf:t,node:e,objectOf:t,oneOf:t,oneOfType:t,shape:t,exact:t,checkPropTypes:o,resetWarningCache:i};return n.PropTypes=n,n}},6042:function(e,t,n){e.exports=n(6747)()},6438:function(e){"use strict";e.exports="SECRET_DO_NOT_PASS_THIS_OR_YOU_WILL_BE_FIRED"}},function(e){e.O(0,[9774,5988,8764,7250,2888,179],(function(){return t=8867,e(e.s=t);var t}));var t=e.O();_N_E=t}]);