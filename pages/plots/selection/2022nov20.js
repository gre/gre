import React from "react";
import Head from "next/head";
import Link from "next/link";
import { getPlots } from "../../../plots";
import { Global } from "../../../components/Global";
import { Container } from "../../../components/Container";
import { Main } from "../../../components/Main";

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
  const url = `/plots/${n}`;

  return (
    <div className="plot">
      <style jsx>{`
        .plot {
          position: relative;
          box-sizing: border-box;
          height: 100vh;
          width: 100vw;
          padding: 10vh 0;
          flex-grow: 1;
        }
        .plot .title {
          position: absolute;
          top: 2vh;
          left: 0;
          width: 100%;
          text-align: center;
          background: white;
          padding: 4px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          width: 100%;
          font-size: min(2vw, 3vh);
        }
        .plot img {
          object-fit: contain;
        }
        .plot em {
          font-size: 0.6em;
        }
      `}</style>

      <Link href={url}>
        <a target="_blank">
          <span className="title">
            Plot #{plot.n} {data.title ? <>{" "}<strong>{data.title}</strong></> : ""} {data.date ? <em> ({data.date})</em> : ""}
            </span>
          <img width="100%" height="100%" src={image} />
        </a>
      </Link>
    </div>
  );
}

const selection = [
  "060",
  "084",
  "118",
  "127",
  "128",
  "203",
  "207",
  "211",
  "218",
  "198",
  "236",
  "241",
  "271",
  "304",
  "312",
  "315",
  "331",
  "367",
  "376",
  "381",
  "398",
  "404",
  "413",
  "453",
  "730",
  "515",
  "560",
  "526",
  "543",
  "561",
  "574",
  "577",
  "606",
  "609",
  "614",
  "626",
  "643",
  "656",
  "670",
  "732",
  "747"
]

export async function getStaticProps() {
  let plots = await getPlots();
  plots = selection.map(s=>plots.find(p => s===p.n));
  return {
    props: { plots },
  };
}

export default function Home({ plots }) {
  const title = `Plots Selection (20 Nov 2022)`;
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
      <iframe width="100%" style={{height: "100vh", padding: "10vh", boxSizing: "border-box"}} src="https://www.youtube.com/embed/zaQ0vKgih6Q" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
      <PlotGrid>
        {plots.map((plot) => (
          <Plot plot={plot} key={plot.n} />
        ))}
      </PlotGrid>
    </Global>
  );
}
