import React, { useState, useEffect } from "react";
import Head from "next/head";
import { getPlots } from "../../../plots";
import { Title } from "../../../components/Title";
import { Container } from "../../../components/Container";
import { Content } from "../../../components/Content";
import { Global } from "../../../components/Global";
import { Main } from "../../../components/Main";
import { Header } from "../../../components/Header";

export async function getStaticPaths() {
  return {
    paths: (await getPlots()).map((p) => {
      return {
        params: {
          n: String(p.n),
        },
      };
    }),
    fallback: false,
  };
}

export async function getStaticProps({ params }) {
  const n = parseInt(params.n, 10);
  const plots = await getPlots();
  const plot = plots.find((p) => parseInt(p.n) === n);
  if (!plot) throw new Error("plot not found!");
  return {
    props: { n, plot },
  };
}

export default function Home({ plot }) {
  const { content, data } = plot;

  return (
    <Global>
      <Container>
        <Head>
          <title>One Day, One Plot – Plot {plot.n}</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <style jsx>{`
              dt {
                font-weight: bold;
              }
              dd {
                display: inline;
                margin: 0;
              }
              dd + dd:before {
                content: ", ";
              }
            `}</style>
            {data.thumbnail ? <img src={data.thumbnail} width="300" /> : null}
            <Title
              text={`Plot #${plot.n}${data.title ? " – " + data.title : ""}`}
            />
            {data.objkts ? (
              <dl>
                <dt>hicetnunc NFTs</dt>
                {data.objkts.map((objkt) => (
                  <dd key={objkt}>
                    <a href={`https://www.hicetnunc.xyz/objkt/${objkt}`}>
                      OBJKT#{objkt}
                    </a>
                  </dd>
                ))}
              </dl>
            ) : null}
          </Header>
          <Content>
            <div
              className="entry-content"
              dangerouslySetInnerHTML={{ __html: content }}
            />

            {data.tweet ? (
              <>
                <blockquote className="twitter-tweet">
                  <a href={data.tweet}></a>
                </blockquote>
                <script
                  async
                  src="https://platform.twitter.com/widgets.js"
                  charSet="utf-8"
                ></script>
              </>
            ) : null}
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
