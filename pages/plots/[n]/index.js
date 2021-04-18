import React from "react";
import Head from "next/head";
import { getPlots } from "../../../plots";
import { Title } from "../../../components/Title";
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

  return (
    <Global>
      <Container>
        <Head>
          <title>One Day, One Plot – Plot {plot.n}</title>
          <link rel="icon" href="/favicon.ico" />
          {highlightAllResources}
        </Head>

        <style jsx>{`
          main {
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
        `}</style>
        <main>
          <aside>
            <h1>
              Plot #{plot.n}
              {data.title ? " – " + data.title : ""}
            </h1>
            {data.thumbnail ? <img src={data.thumbnail} width="100%" /> : null}

            <div className="properties">
              <span />
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
          </div>
        </main>
        <HighlightAll />
      </Container>
    </Global>
  );
}
