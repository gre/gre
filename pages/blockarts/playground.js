import Head from "next/head";
import { useState, useRef, useEffect } from "react";
import useDimensions from "react-cool-dimensions";
import { proxy, useSnapshot } from "valtio";
import { EthBlockArtVisual } from "../../components/EthBlockArtVisual";
import { LiveFooter } from "../../components/LiveFooter";
import { SubTitle } from "../../components/ShaderdaySubTitle";
import { Title } from "../../components/Title";
import { SourceCodeFooter } from "../../components/SourceCodeFooter";
import { Container } from "../../components/Container";
import { Global } from "../../components/Global";
import { Main } from "../../components/Main";
import { Header } from "../../components/Header";
import Sidebar from "../../components/ethblockart/Sidebar";
import * as BlockArt from "../../blockarts/current";
import blocks from "../../blockarts/blocks";

const store = proxy({
  ...BlockArt.styleMetadata,
});

let id = 1;
async function rpc(method, params = []) {
  const r = await fetch(
    `https://eth-mainnet.alchemyapi.io/v2/Z1sYlBWMorPqRSMevhoOJgcgweL0_rE2`,
    {
      method: "POST",
      body: JSON.stringify({
        id: id++,
        jsonrpc: "2.0",
        method,
        params,
      }),
    }
  );
  const res = await r.json();
  return res.result;
}

function sortBlocks() {
  blocks.sort((a, b) => parseInt(b.number) - parseInt(a.number));
}

function useBlock(blockNumber) {
  const [block, setBlock] = useState(blocks[0]);
  useEffect(() => {
    let cancelled;
    async function main() {
      const block = await rpc("eth_getBlockByNumber", [
        "0x" + blockNumber.toString(16),
        true,
      ]);
      if (!block || cancelled) return;
      setBlock(block);
    }
    main();
    return () => {
      cancelled = true;
    };
  }, [blockNumber]);
  return block;
}

function useCurrentBlockNumber() {
  const [blockNumber, setBlockNumber] = useState(0);
  useEffect(() => {
    if (BlockArt.styleMetadata.debug_noRefresh) return;
    async function refresh() {
      const blockNumber = await rpc("eth_blockNumber");
      if (!blockNumber) return;
      setBlockNumber(parseInt(blockNumber));
    }
    const interval = setInterval(refresh, 3000);
    refresh();
    return () => clearInterval(interval);
  }, []);
  return blockNumber;
}

function useRandomLoadingBlocks(count = 10, interval = 120000, delay = 100) {
  useEffect(() => {
    if (BlockArt.styleMetadata.debug_noRefresh) return;
    async function refresh() {
      const blockNumber = await rpc("eth_blockNumber");
      if (!blockNumber) return;
      const n = parseInt(blockNumber);
      const pick = Math.floor(n * Math.random());
      for (let i = pick; i >= 1 && i > pick - count; i--) {
        if (blocks.some((b) => b && parseInt(b.number) === i)) continue;
        const block = await rpc("eth_getBlockByNumber", [
          "0x" + i.toString(16),
          true,
        ]);
        if (!block) return;
        blocks.push(block);
        sortBlocks();
        await new Promise((success) => setTimeout(success, delay));
      }
      console.log(blocks);
    }
    const interval = setInterval(refresh, 60000);
    refresh();

    return () => clearInterval(interval);
  }, []);
}

const seed = BlockArt.styleMetadata.options.seed || 0;
const shouldAutoSeed = seed < 0 && -seed % 2 == 0;
const shouldBeMinimal = seed < 0 && (-seed >> 1) % 2 == 0;

export default function Home() {
  const { ref, width, height } = useDimensions({});
  const [blockNumber, setBlockNumber] = useState(0);
  const currentBlockNumber = useCurrentBlockNumber();
  const snap = useSnapshot(store);
  const attributesRef = useRef();
  const block = useBlock(blockNumber || currentBlockNumber);
  // useCurrentBlock();
  // useRandomLoadingBlocks();

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
                block={block}
                attributesRef={attributesRef}
              />
            </div>
            <div
              style={shouldBeMinimal ? { position: "fixed", right: 0 } : null}
            >
              <Sidebar
                block={block}
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
