import React from "react";
import Head from "next/head";
import { getPlots } from "../../../plots";
import { Visual } from "../../../components/Visual";
import { LiveFooter } from "../../../components/LiveFooter";
import { SubTitle } from "../../../components/SubTitle";
import { Title } from "../../../components/Title";
import { SourceCodeFooter } from "../../../components/SourceCodeFooter";
import { Container } from "../../../components/Container";
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
  const { content } = plot;
  return (
    <Global>
      <Container>
        <Head>
          <title>One Day, One Plot – Plot {plot.n}</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <Header>
            <Title text="One Day, One Plot" />
          </Header>

          <div
            className="entry-content"
            dangerouslySetInnerHTML={{ __html: content }}
          />
        </Main>
      </Container>
    </Global>
  );
}
