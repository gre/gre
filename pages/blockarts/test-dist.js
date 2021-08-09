import React, { useEffect, useState } from "react"
import Head from "next/head";
import dynamic from 'next/dynamic';
import { useRef } from "react";
import { Container } from "../../components/Container";
import { Global } from "../../components/Global";
import { Main } from "../../components/Main";
import { useBlock, useCurrentBlockNumber } from "./playground";

const BlockStyle = typeof window === "undefined" ? "div" : dynamic(() => import("../../dist/main.js"));

export default function Home() {
  const currentBlockNumber = useCurrentBlockNumber();
  const block = useBlock(currentBlockNumber);
  const attributesRef = useRef({});
  const [attr, setAttr] = useState({});
  useEffect(() => {
      setTimeout(() => {
        setAttr(attributesRef.current());
      }, 1000);
  }, [attributesRef]);

  if (!block) return null;
  return (
    <Global>
      <Container>
        <Head>
          <title>ethblock.art preview dist</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
            <BlockStyle
                block={block}
                width={400}
                height={400}
                mod1={0.5}
                mod2={0.5}
                mod3={0.5}
                mod4={0.5}
                attributesRef={attributesRef}
              />
              <pre>
              {attr?.attributes?.map(o => o.trait_type+": "+o.value).join("\n")}
              </pre>
        </Main>
      </Container>
    </Global>
  );
}
