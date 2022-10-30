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
        marginTop: -4,
        marginBottom: "20px",
        padding: "4px 16px",
        background: "#000",
        color: "white",
      }}
    >
      <p>
        @greweb loves exploring the beauty of noise through many algorithms,
        notably using shaders and plotters.
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
          display: flex;
          flex-wrap: wrap;
        }
      `}</style>
    </div>
  );
}

export function Plot({ plot }) {
  const { data, n } = plot;
  const { image } = data;
  const title = `Plot #${plot.n} ${data.title ? " – " + data.title : ""}`;
  const url = `/plots/${n}`;
  const thumbnail =
    image && image.endsWith(".jpg")
      ? image.replace(/\.([^.]+)$/, "-thumbnail.$1")
      : null;

  return (
    <div className="plot">
      <style jsx>{`
        .plot {
          position: relative;
          height: 200px;
          padding: 2px;
          flex-grow: 1;
        }
        .plot:hover .title {
          opacity: 1;
          transition: opacity 200ms;
        }
        .plot .title {
          opacity: 0;
          position: absolute;
          left: 0;
          bottom: 0;
          display: inline-block;
          background: white;
          padding: 4px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          width: 100%;
          font-size: 12px;
        }
        .plot img {
          object-fit: cover;
        }
      `}</style>

      <Link href={url}>
        <a title={title}>
          <span className="title">{title}</span>
          <img width="100%" height="100%" src={thumbnail || image} />
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

            <PlotGrid>
              {plots.map((plot) => (
                <Plot plot={plot} key={plot.n} />
              ))}
            </PlotGrid>

            <PlottingHeader />
            <PlottingSectionVideos />


            <footer>
              <MeBlock />
            </footer>
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
