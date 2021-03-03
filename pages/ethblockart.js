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
    let i = setInterval(() => {
      let m = mods.find((m) => m.key === "seed");
      if (m) {
        m.set(Math.random());
      }
    }, 1000);
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
          <Header>
            <h1>{BlockArt.styleMetadata.name}</h1>
            <p style={{ maxWidth: 800 }}>
              {BlockArt.styleMetadata.description}
            </p>
          </Header>
          <div style={{ display: "flex", flexDirection: "row" }}>
            <div
              ref={ref}
              style={{
                width: "60vw",
                height: "60vw",
              }}
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
            <Sidebar
              blocks={blocks}
              blockNumber={blockNumber}
              attributes={attributesRef.current || {}}
              mods={mods}
              handleBlockChange={(i) => setBlockNumber(i)}
            />
          </div>
        </Main>
      </Container>
    </Global>
  );
}
