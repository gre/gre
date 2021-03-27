import Head from "next/head";
import { useState, useRef, useEffect } from "react";
import useDimensions from "react-cool-dimensions";
import { proxy, useProxy } from "valtio";
import { EthBlockArtVisual } from "../components/EthBlockArtVisual";
import { LiveFooter } from "../components/LiveFooter";
import { SubTitle } from "../components/SubTitle";
import { Title } from "../components/Title";
import { SourceCodeFooter } from "../components/SourceCodeFooter";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import Sidebar from "../components/ethblockart/Sidebar";
import * as BlockArt from "../blockarts/current";
import blocks from "../blocks";

const store = proxy({
  ...BlockArt.styleMetadata,
});

const seed = BlockArt.styleMetadata.options.seed || 0;
const shouldAutoSeed = seed < 0 && -seed % 2 == 0;
const shouldBeMinimal = seed < 0 && (-seed >> 1) % 2 == 0;

export default function Home() {
  const { ref, width, height } = useDimensions({});
  const [blockNumber, setBlockNumber] = useState(1);
  const snap = useProxy(store);
  const attributesRef = useRef();

  const mods = Object.keys(store.options).map((k) => {
    return {
      key: k,
      value: snap.options[k],
      set: (v) => {
        store.options[k] = v;
      },
    };
  });

  useEffect(() => {
    const m = mods.find((m) => m.key === "seed");
    const i = setInterval(() => {
      if (m && shouldAutoSeed) {
        m.set(Math.random());
      }
    }, 5000);
    return () => {
      clearInterval(i);
    };
  }, [mods]);

  return (
    <Global>
      <Container>
        <Head>
          <title>ethblock.art sandbox</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          {shouldBeMinimal ? null : (
            <Header>
              <h1>{BlockArt.styleMetadata.name}</h1>
              <p style={{ maxWidth: 800 }}>
                {BlockArt.styleMetadata.description}
              </p>
            </Header>
          )}
          <div style={{ display: "flex", flexDirection: "row" }}>
            <div
              ref={ref}
              style={
                shouldBeMinimal
                  ? {
                      width: "100vw",
                      height: "100vh",
                    }
                  : {
                      width: "60vw",
                      height: "60vw",
                    }
              }
            >
              <EthBlockArtVisual
                width={width}
                height={height}
                BlockStyle={BlockArt.default}
                values={snap.options}
                block={blocks[blockNumber]}
                attributesRef={attributesRef}
              />
            </div>
            <div
              style={shouldBeMinimal ? { position: "fixed", right: 0 } : null}
            >
              <Sidebar
                blocks={blocks}
                blockNumber={blockNumber}
                attributesRef={attributesRef}
                mods={mods}
                handleBlockChange={(i) => setBlockNumber(i)}
              />
            </div>
          </div>
          {!shouldBeMinimal ? null : (
            <div
              style={{
                color: "#666",
                position: "fixed",
                bottom: 0,
                padding: 10,
              }}
            >
              <span>
                {width}x{height}
              </span>{" "}
              <strong>{BlockArt.styleMetadata.name}</strong>{" "}
              <em style={{ fontSize: "0.8em" }}>
                {BlockArt.styleMetadata.description}
              </em>
            </div>
          )}
        </Main>
      </Container>
    </Global>
  );
}
