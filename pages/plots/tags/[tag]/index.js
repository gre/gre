import React from "react";
import Head from "next/head";
import { getPlots } from "../../../../plots";
import { getTagContent } from "../../../../plots/tags";
import { Global } from "../../../../components/Global";
import { Container } from "../../../../components/Container";
import { Main } from "../../../../components/Main";
import { Header } from "../../../../components/Header";
import { Title } from "../../../../components/Title";
import MeBlock from "../../../../components/MeBlock";
import { Plot, Content, PlottingHeader } from "../..";
import { PlottingSectionVideos } from "../../nft";

function PlotGrid({ children }) {
  return (
    <div className="content">
      {children}
      <style jsx>{`
        .content {
          display: flex;
          flex-wrap: wrap;
          max-width: 1200px;
          margin: 0 auto;
        }
      `}</style>
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
  const tagContent = await getTagContent(params.tag).then(r => r, () => null);
  return {
    props: { tag: params.tag, plots, tagContent },
  };
}

export default function Home({ tag, plots, tagContent }) {
  console.log(tagContent)
  const title = `Plots with tag '${tag}'`;

  const firstThumbnail = plots.map((p) => p.data.image).filter(Boolean)[0];

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
            <style jsx>{`
              .tag-content {
                max-width: 1200px;
                margin: 0px auto;
                padding: 10px 20px;
                background: #000;
                color: #fff;
              }

          footer {
            margin: 20px;
            padding: 20px;
            border-top: 2px solid #000;
          }
      `}</style>

            {tagContent ? <div
              className="tag-content"
              dangerouslySetInnerHTML={{ __html: tagContent.content }}
            /> : null}
            <PlotGrid>
              {plots.map((plot) => (
                <Plot height={400} plot={plot} key={plot.n} />
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
