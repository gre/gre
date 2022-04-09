import React from "react";
import Head from "next/head";
import { getPlots } from "../../../../plots";
import { Global } from "../../../../components/Global";
import { Container } from "../../../../components/Container";
import { Main } from "../../../../components/Main";
import { Header } from "../../../../components/Header";
import { Title } from "../../../../components/Title";
import MeBlock from "../../../../components/MeBlock";
import { Plot, PlotGrid, Content } from "../..";

export async function getStaticPaths() {
  const plots = await getPlots();
  const tags = Array.from(new Set(plots.flatMap((p) => p.data.tags || [])));
  return {
    paths: tags.map((tag) => {
      return {
        params: {
          tag,
        },
      };
    }),
    fallback: false,
  };
}

export async function getStaticProps({ params }) {
  const all = await getPlots();
  const plots = all.filter((p) => p.data.tags?.includes(params.tag));
  if (plots.length === 0) throw new Error("no such tag");
  return {
    props: { tag: params.tag, plots },
  };
}

export default function Home({ tag, plots }) {
  const title = `Plots with tag '${tag}'`;

  const firstThumbnail = plots.map((p) => p.data.thumbnail).filter(Boolean)[0];

  return (
    <Global>
      <Head>
        <title>greweb.me – {title}</title>
        <link rel="icon" href="/favicon.ico" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="author" content="Gaëtan Renaudeau" />
        <meta name="keywords" content={"plotter, plots, " + tag} />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:site" content="@greweb" />
        <meta name="twitter:title" content={title} />
        <meta name="twitter:creator" content="@greweb" />
        {firstThumbnail ? (
          <>
            <meta
              name="twitter:image"
              content={`http://greweb.me${firstThumbnail}`}
            />
            <link rel="image_src" href={`http://greweb.me${firstThumbnail}`} />
            <meta
              property="og:image"
              content={`http://greweb.me${firstThumbnail}`}
            />
          </>
        ) : null}
      </Head>
      <Container>
        <Main>
          <Header>
            <Title withBreadcrumb text={title} />
          </Header>

          <Content>
            <PlotGrid>
              {plots.map((plot) => (
                <Plot plot={plot} key={plot.n} />
              ))}
            </PlotGrid>

            <footer>
              <MeBlock />
            </footer>
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
