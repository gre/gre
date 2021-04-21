import React from "react";
import Head from "next/head";
import { getPlots } from "../../../plots";
import { Container } from "../../../components/Container";
import { Global } from "../../../components/Global";
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
  const plot = plots.find((p) => parseInt(p.n) === nNumber);
  if (!plot) throw new Error("plot not found!");
  return {
    props: { n: params.n, plot },
  };
}

export default function Home({ n, plot }) {
  const { content, data } = plot;
  const { thumbnail } = data;
  const title = `Plot #${plot.n} ${data.title ? " – " + data.title : ""}`;

  return (
    <Global>
      <Container>
        <Head>
          <title>greweb.me – {title}</title>
          <link rel="icon" href="/favicon.ico" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <meta name="twitter:card" content="summary_large_image" />
          <meta name="twitter:site" content="@greweb" />
          <meta name="twitter:title" content={title} />
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
            align-self: stretch;
            display: flex;
            flex-direction: row;
          }
          aside {
            flex: 0.8;
            padding: 10px;
            text-align: center;
          }
          h1 {
          }
          div.content {
            flex: 1;
            max-width: 500px;
            padding: 10px;
          }
          @media screen and (max-width: 900px) {
            main {
              flex-direction: column;
            }
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
          <aside>
            <h1>{title}</h1>
            {thumbnail ? <img src={thumbnail} width="100%" /> : null}

            <div className="properties">
              <span />
              {data.objkts ? (
                <dl>
                  <dt>hicetnunc NFTs</dt>
                  {data.objkts.map((objkt) => (
                    <dd key={objkt}>
                      <a
                        target="_blank"
                        href={`https://www.hicetnunc.xyz/objkt/${objkt}`}
                        rel="noreferrer"
                      >
                        OBJKT#{objkt}
                      </a>
                    </dd>
                  ))}
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
                <dt>Rust sourcecode</dt>
                <dd>
                  <a
                    href={`https://github.com/gre/gre/blob/master/plots/examples/${n}/main.rs`}
                  >
                    main.rs
                  </a>
                </dd>
              </dl>
              <span />
            </div>
          </aside>
          <div className="content">
            <div
              className="entry-content"
              dangerouslySetInnerHTML={{ __html: content }}
            />

            {data.tweet ? (
              <>
                <blockquote className="twitter-tweet" data-conversation="none">
                  <a href={data.tweet}></a>
                </blockquote>
                <script
                  async
                  src="https://platform.twitter.com/widgets.js"
                  charSet="utf-8"
                ></script>
              </>
            ) : null}
          </div>
        </main>
        <HighlightAll />
      </Container>
    </Global>
  );
}
