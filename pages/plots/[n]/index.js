import React, { useEffect } from "react";
import Head from "next/head";
import Link from "next/link";
import { getPlots } from "../../../plots";
import { SubTitle } from "../../../components/PlotSubTitle";
import { Global } from "../../../components/Global";
import MeBlock from "../../../components/MeBlock";
import {
  HighlightAll,
  highlightAllResources,
} from "../../../components/HighlightAll";

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
  const nNumber = parseInt(params.n, 10);
  const plots = await getPlots();
  const plot = plots.find((p) => parseInt(p.n, 10) === nNumber);
  if (!plot) throw new Error("plot not found!");
  const prev = plots.find((p) => parseInt(p.n, 10) === nNumber - 1) || null;
  const next = plots.find((p) => parseInt(p.n, 10) === nNumber + 1) || null;
  return {
    props: { n: params.n, plot, prev, next },
  };
}

export default function Home({ plot, prev, next }) {
  const { content, data, rustFile, sourceURL } = plot;
  const { thumbnail } = data;
  const title = `Plot #${plot.n} ${data.title ? " – " + data.title : ""}`;
  const description = data.description || "";

  useEffect(() => {
    if (window.twttr) {
      window.twttr.widgets.load();
    }
  }, [plot.n]);

  return (
    <Global>
      <Head>
        <title>greweb.me – {title}</title>
        <link rel="icon" href="/favicon.ico" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="author" content="Gaëtan Renaudeau" />
        <meta name="description" content={description} />
        <meta name="keywords" content={data.tags?.join(", ")} />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:site" content="@greweb" />
        <meta name="twitter:title" content={title} />
        <meta name="twitter:description" content={description} />
        <meta name="twitter:creator" content="@greweb" />
        {thumbnail ? (
          <>
            <meta
              name="twitter:image"
              content={`http://greweb.me${thumbnail}`}
            />
            <link rel="image_src" href={`http://greweb.me${thumbnail}`} />
            <meta
              property="og:image"
              content={`http://greweb.me${thumbnail}`}
            />
          </>
        ) : null}

        {highlightAllResources}
      </Head>

      <style jsx>{`
        main {
          max-width: 680px;
          margin: 0px auto;
        }
        header {
          flex: 1;
          padding: 10px;
          text-align: center;
        }
        h1 {
        }
        div.content {
          flex: 1;
          padding: 10px;
        }
        div.properties {
          display: flex;
          flex-direction: row;
          justify-content: space-around;
        }
        dl {
          display: inline-block;
          max-width: 300px;
        }
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
        .content {
          padding: 10px;
        }
        .content a {
          text-decoration: underline;
        }
        footer {
          margin-top: 20px;
          padding-top: 10px;
          border-top: 2px solid #000;
        }
      `}</style>
      <style>
        {
          /* TMP hack */ `
          pre {
            overflow-y: hidden;
            overflow: scroll;
            max-width: 90vw;
          }`
        }
      </style>
      <main>
        <header>
          <SubTitle plot={plot} prev={prev} next={next} />
          <div className="properties">
            <span />
            {data.objkts ? (
              <dl>
                <dt>hicetnunc NFTs</dt>
                {data.objkts.map((objkt) => (
                  <dd key={objkt}>
                    <a
                      target="_blank"
                      href={`https://www.objkt.com/objkt/${objkt}`}
                      rel="noreferrer"
                    >
                      OBJKT#{objkt}
                    </a>
                  </dd>
                ))}
              </dl>
            ) : null}
            {data.nftGenerator ? (
              <dl>
                <dt>NFT Generator</dt>
                <dd>
                  <a target="_blank" href={data.nftGenerator} rel="noreferrer">
                    LINK
                  </a>
                </dd>
              </dl>
            ) : null}
            {data.nft ? (
              <dl>
                <dt>NFT</dt>
                <dd>
                  <a target="_blank" href={data.nft} rel="noreferrer">
                    LINK
                  </a>
                </dd>
              </dl>
            ) : null}
            {data.plotterfiles ? (
              <dl>
                <dt>Plotterfiles</dt>
                {data.plotterfiles.map((p) => (
                  <dd key={p}>
                    <a target="_blank" href={p} rel="noreferrer">
                      link
                    </a>
                  </dd>
                ))}
              </dl>
            ) : null}
            <dl>
              <dt>Sourcecode</dt>
              <dd>
                <a href={sourceURL}>{rustFile}</a>
              </dd>
            </dl>
            {data.tags ? (
              <dl>
                <dt>Tags</dt>
                {data.tags.map((t) => (
                  <dd key={t}>
                    <Link href={`/plots/tags/${t}`}>
                      <a>{t}</a>
                    </Link>
                  </dd>
                ))}
              </dl>
            ) : null}
            <span />
          </div>
          {thumbnail ? <img src={thumbnail} width="100%" /> : null}
        </header>
        <div className="content" key={plot.n}>
          <div
            className="entry-content"
            dangerouslySetInnerHTML={{ __html: content }}
          />

          {data.tweet ? (
            <blockquote className="twitter-tweet" data-conversation="none">
              <a href={data.tweet}></a>
            </blockquote>
          ) : null}

          <footer>
            <MeBlock />
          </footer>
        </div>
      </main>
      <HighlightAll />
      <script
        async
        src="https://platform.twitter.com/widgets.js"
        charSet="utf-8"
      ></script>
    </Global>
  );
}
