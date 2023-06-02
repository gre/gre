import React from "react";
import Head from "next/head";
import Link from "next/link";
import { useRouter } from 'next/router'
import QRCode from "react-qr-code";
import { getPlots } from "../../../plots";
import { Global } from "../../../components/Global";

function Content({ children }) {
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

function PlotGrid({ children }) {
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

function Plot({ plot }) {
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
          font-size: min(2vw, 3vh);
        }
        .plot .title {
          position: absolute;
          top: 3vh;
          left: 0;
          width: 100%;
          text-align: center;
          background: white;
          padding: 4px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          width: 100%;
        }
        .plot .foot {
          position: absolute;
          bottom: 5vh;
          left: 0;
          width: 100%;
          text-align: center;
          background: white;
          padding: 4px;
          width: 100%;
        }
        .plot img {
          object-fit: contain;
        }
        .plot em {
          font-weight: 100;
        }
        .plot .tags {
          margin-top: 10px;
          font-size: 0.8em;
          opacity: 0.4;
        }
        .plot .tags > * {
          padding: 0 0.2em;
        }

      `}</style>

      <Link href={url}>
        <a target="_blank">
          <span className="title">
            Plot #{plot.n} {data.title ? <>{" "}<strong>{data.title}</strong></> : ""} {data.date ? <em> ({data.date})</em> : ""}
          </span>
          <img width="100%" height="100%" src={image} />
          <span className="foot">
            {" "}{data.tags ? <div className="tags">{data.tags.map(t => <span key={t}>
              <a target="_blank" href={`/plots/tags/${t}`}>
                #{t}
              </a>
            </span>)}</div> : ""}
          </span>
        </a>
      </Link>
    </div>
  );
}

function PrintPlotGrid({ children }) {
  return (
    <div className="content">
      {children}
      <style jsx>{`
@page { 
  size: auto;
  margin: 5mm;  
} 
.content {
  display: flex;
  flex-wrap: wrap;
  width: 100%;
}
      `}</style>
    </div>
  );
}

function PrintPlot({ plot }) {
  const { data, n } = plot;

  return (
    <div className="printplot">
      <style jsx>{`
.printplot {
  position: relative;
  font-size: 10pt;
  width: 104mm;
  height: 54mm;
  padding: 1em;
  box-sizing: border-box;
  border: 1px solid #eee;
  display: flex;
  flex-direction: row;
  break-inside: avoid;
}
.printplot .content {
  display: flex;
  flex-direction: column;
  flex: 1;
  font-size: 8pt;
}
.printplot .title {
  font-weight: bold;
  font-size: 11pt;
  margin-bottom: 0.2em;
}
.printplot .subtitle {
  font-style: italic;
  margin-bottom: 1em;
}
.printplot .description {
}
.printplot .url {
  display: flex;
  flex-direction: column;
  font-size: 5pt;
  text-align: center;
  padding-left: 1em;
  margin-top: 32pt;
}
.printplot .url span {
  margin-top: 1em;
}
      `}</style>
      <div className="content">
        <div className="title">
          {data.title}
        </div>
        <div className="subtitle">
          @greweb – {data.date}
        </div>
        <div className="description">
          {data.frDescription}
        </div>
      </div>
      <div className="url">
        <QRCode size={100} value={`https://greweb.me/plots/${n}`} />
        <span>{`https://greweb.me/plots/`}<strong>{plot.n}</strong></span>
      </div>
    </div>
  )
}

const selection = [
  "198",
  "207",
  "304",
  "403",
  "515",
  "574",
  "577",
  "621",
  "670",
  "839",
  "841",
  "884",
  "1058",
  "1059",
  "1063",
]

export async function getStaticProps() {
  let plots = await getPlots();
  plots = selection.map(s => plots.find(p => s === p.n));
  return {
    props: { plots },
  };
}

export default function Home({ plots }) {
  const router = useRouter();
  const title = `Plots Selection (8 june 2023)`;
  const firstThumbnail = plots.map((p) => p.data.thumbnail).filter(Boolean)[0];
  const print = router.query.print === "true"

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
      {!print ?
        <PlotGrid>
          {plots.map((plot) => (
            <Plot plot={plot} key={plot.n} />
          ))}
        </PlotGrid> : <PrintPlotGrid>
          {plots.map((plot) => (
            <PrintPlot plot={plot} key={plot.n} />
          ))}
        </PrintPlotGrid>}
    </Global>
  );
}
