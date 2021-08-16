// @flow
import React, { useEffect, useRef, useState } from "react";
import { Shaders, Node, GLSL, Uniform } from "gl-react";
import { Surface } from "gl-react-dom";
import Head from "next/head";
import useDimensions from "react-cool-dimensions";
import { Container } from "../../components/Container";
import { Global } from "../../components/Global";
import { Main } from "../../components/Main";
import { Header } from "../../components/Header";

const delay = (ms) => new Promise(s => setTimeout(s, ms))

const shaders = Shaders.create({
    main: {
      frag: GLSL`
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
    `
  }
})

function useTime() {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    let h;
    function loop(t) {
      h = requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    h = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(h);
  }, []);
  return time;
}

const Render = ({ width, height, surfaceRef }) => {
  const time = useTime();
  return (
    <Surface webglContextAttributes={{ preserveDrawingBuffer: true }} width={width} height={height} ref={surfaceRef}>
        <Node shader={shaders.main} uniforms={{
            time,
            resolution: Uniform.Resolution
        }} />
    </Surface>
  );
}

const SomethingSlow = ({ burn }) => {
  const t = Date.now();
  while(Date.now() - t < burn);
  return null
}

export default function Home() {
  const { observe, width, height } = useDimensions({});
  const surfaceRef = useRef()
  const [incr, setIncr] = useState(0)
  const [incr2, setIncr2] = useState(0)
  const [expand, setExpand] = useState(false)
  const [snap, setSnap] = useState()

  useEffect(() => {
    const capture = async () => {
      try {
        setExpand(true);
        await delay(1000);
        const img = await surfaceRef.current.captureAsBlob();
        const url = URL.createObjectURL(img)
        await delay(300)
        setExpand(false)
        setSnap({ img, url });
      }
      catch (e) {
        setExpand(false)
        console.error(e)
      }
    }

    capture()

  }, [incr]);

  return (
    <Global>
      <Container>
        <Head>
          <title>stress test</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <SomethingSlow burn={950} />
          <Header>
            <h1>Mandelglitch slideshow experiment</h1>
          </Header>
          <button onClick={()=>setIncr(incr=>incr+1)}>SNAP</button>
          <button onClick={()=>setIncr2(incr=>incr+1)}>BURNCPU({incr2})</button>
          {snap ? <div style={{border:"4px solid red"}}><img src={snap.url} width={100} /></div> : null}
          <div style={{ display: "flex", flexDirection: "row" }}>
            <div ref={observe} style={{ width: expand ? "80vw" : "20vw", height: expand ? "80vw" : "20vw" }}>
              <Render surfaceRef={surfaceRef} width={width} height={height} />
            </div>
            <SomethingSlow burn={100} />
          </div>
        </Main>
      </Container>
    </Global>
  );
}
