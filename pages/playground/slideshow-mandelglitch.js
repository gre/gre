// @flow
import React, { useEffect, useState } from "react";
import Head from "next/head";
import { useMemo } from "react";
import useDimensions from "react-cool-dimensions";
import { Container } from "../../components/Container";
import { Global } from "../../components/Global";
import { Main } from "../../components/Main";
import { Header } from "../../components/Header";
import { Surface } from "gl-react-dom";
import { useRandomBlocks } from "../blockarts/playground";
import { MandelglitchTransition } from "../../blockarts/animations/MandelglitchAnimateMints";

export default function Home() {
  const { observe, width, height } = useDimensions({});

  // these is a quick internal trick to get some random blocks & fake mints
  const blocks = useRandomBlocks();
  const mints = useMemo(
    () =>
      blocks.map((block) => {
        return {
          block,
          mod1: Math.random(),
          mod2: Math.random(),
          mod3: Math.random(),
        };
      }),
    [blocks]
  );

  // this is simply going to rotate on "mints"
  // we could also "react on click"
  // lot of possibility... i prefered to kept the engine out of MandelglitchTransition
  // so the external is in control.
  // MandelglitchTransition is going to impl transition between the props (react-spring)
  const [count, setCount] = useState(0);
  const delay = 3000;
  useEffect(() => {
    const interval = setInterval(() => {
      setCount((c) => c + 1);
    }, delay);
    return () => clearInterval(interval);
  }, [delay]);
  const mint = mints[count % mints.length];

  if (!mint) return null;
  return (
    <Global>
      <Container>
        <Head>
          <title>Slideshow mandelglitch</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <h1>Mandelglitch slideshow experiment</h1>
          </Header>
          <div style={{ display: "flex", flexDirection: "row" }}>
            <div ref={observe} style={{ width: "60vw", height: "60vw" }}>
              <Surface width={width} height={height}>
                <MandelglitchTransition mint={mint} />
              </Surface>
            </div>
          </div>
        </Main>
      </Container>
    </Global>
  );
}
