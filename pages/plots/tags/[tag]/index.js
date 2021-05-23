import React from "react";
import Head from "next/head";
import Link from "next/link";
import { getPlots } from "../../../../plots";
import { Global } from "../../../../components/Global";
import { Container } from "../../../../components/Container";
import { Main } from "../../../../components/Main";
import { Content } from "../../../../components/Content";
import { Header } from "../../../../components/Header";
import { Title } from "../../../../components/Title";
import MeBlock from "../../../../components/MeBlock";

function Plot({ plot }) {
  const { data, n } = plot;
  const { thumbnail } = data;
  const title = `Plot #${plot.n} ${data.title ? " – " + data.title : ""}`;
  const description = data.description || "";
  const url = `/plots/${n}`;
  return (
    <div className="plot">
      <style jsx>{`
        .plot {
          margin-bottom: 40px;
          padding-bottom: 40px;
          border-bottom: 4px solid black;
        }
      `}</style>

      <Link href={url}>
        <a>
          <h2>{title}</h2>
          <img src={thumbnail} width="100%" />
        </a>
      </Link>
      <em>{description}</em>
    </div>
  );
}

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
            {plots.map((plot) => (
              <Plot plot={plot} key={plot.n} />
            ))}

            <footer>
              <MeBlock />
            </footer>
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
