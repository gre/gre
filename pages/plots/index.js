import React from "react";
import Head from "next/head";
import Link from "next/link";
import { getPlots } from "../../plots";
import { Global } from "../../components/Global";
import { Container } from "../../components/Container";
import { Main } from "../../components/Main";
import { Header } from "../../components/Header";
import { Title } from "../../components/Title";
import MeBlock from "../../components/MeBlock";
import { PlottingSectionVideos } from "./nft";

export function PlottingHeader() {
  return (
    <footer
      style={{
        textAlign: "center",
        fontStyle: "italic",
        fontSize: "20px",
        margin: "20px 0",
        padding: "4px 16px",
        background: "#000",
        color: "white",
      }}
    >
      <p>
        @greweb has been doing generative art for many years, shaders, and more
        recently fountain pens robot plotting!
      </p>

      <p>
        His work is about exploring the beauty of noise through many algorithms.
      </p>
    </footer>
  );
}

export function Content({ children }) {
  return (
    <div className="content">
      {children}
      <style jsx>{`
        .content {
          padding: 0 20px;
        }
      `}</style>
    </div>
  );
}

export function PlotGrid({ children }) {
  return (
    <div className="content">
      {children}
      <style jsx>{`
        .content {
          display: grid;
          grid-template-columns: repeat(3, 1fr);
        }
        @media (max-width: 860px) {
          .content {
            grid-template-columns: repeat(2, 1fr);
          }
        }
        @media (max-width: 540px) {
          .content {
            grid-template-columns: repeat(1, 1fr);
          }
        }
      `}</style>
    </div>
  );
}

export function Plot({ plot }) {
  const { data, n } = plot;
  const { thumbnail } = data;
  const title = `Plot #${plot.n} ${data.title ? " – " + data.title : ""}`;
  const url = `/plots/${n}`;
  return (
    <div className="plot">
      <style jsx>{`
        .plot {
          position: relative;
          min-height: 100px;
          padding: 2px;
        }
        .plot .title {
          position: absolute;
          left: 0;
          top: 0;
          padding: 4px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          width: 100%;
          font-size: 12px;
          background: #fff;
        }
        .plot img {
          object-fit: cover;
        }
      `}</style>

      <Link href={url}>
        <a title={title}>
          <span className="title">{title}</span>
          <img src={thumbnail} width="100%" height="100%" />
        </a>
      </Link>
    </div>
  );
}

export async function getStaticProps() {
  const plots = await getPlots();
  return {
    props: { plots },
  };
}

export default function Home({ plots }) {
  const title = `Plots`;
  const firstThumbnail = plots.map((p) => p.data.thumbnail).filter(Boolean)[0];

  return (
    <Global>
      <Head>
        <title>greweb.me – {title}</title>
        <link rel="icon" href="/favicon.ico" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="author" content="Gaëtan Renaudeau" />
        <meta name="keywords" content={"plotter, plots"} />
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
            <PlottingSectionVideos />
            <PlottingHeader />

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
