import React from "react";
import Head from "next/head";
import Link from "next/link";
import { Leva } from "leva";
import { Title } from "../components/Title";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { Visual } from "../components/Visual";
import { getDays } from "../shaderdays";
import { getAllPosts } from "../posts";
import { getPlots } from "../plots";
import me from "../me";

export async function getStaticProps() {
  const posts = await getAllPosts();
  const plots = await getPlots();
  return {
    props: { posts, plots },
  };
}

export default function Home({ posts, plots }) {
  const days = getDays()
    .slice()
    .sort((a, b) => b.n - a.n);

  const siteURL = me.thumbnailDomain;
  const { title, description, thumbnail } = me;

  return (
    <Global>
      <Container>
        <Head>
          <title>{title}</title>
          <link rel="icon" href="/favicon.ico" />
          <meta name="twitter:card" content="summary" />
          <meta name="twitter:site" content="@greweb" />
          <meta name="twitter:title" content={title} />
          <meta name="og:title" content={title} />
          <meta name="twitter:description" content={description} />
          <meta name="twitter:creator" content="@greweb" />
          <meta name="og:image" content={`${siteURL}/${thumbnail}`} />
          <meta name="twitter:image" content={`${siteURL}/${thumbnail}`} />
          <link rel="image_src" href={`${siteURL}/${thumbnail}`} />
        </Head>

        <Leva hidden />
        <Main>
          <style jsx>{`
            blockquote {
              max-width: 500px;
            }
            dl {
            }
            dt {
              margin-top: 1.6em;
              margin-bottom: 0.8em;
              font-weight: bold;
            }
            dd.inline {
              display: inline;
            }
            dd.inline + dd.inline {
              margin: 0;
            }
            dd.inline + dd.inline:before {
              content: ", ";
            }
            .social {
              padding: 0px;
              list-style: none;
            }
            .social img {
              vertical-align: middle;
            }
            .social li {
              padding: 5px 0;
            }
            .header-top {
              display: flex;
              flex-direction: row;
            }
            .avatarbox {
              padding-right: 20px;
            }
            .content {
              max-width: 600px;
              padding-top: 10px;
              margin-top: 10px;
              border-top: 4px solid #000;
            }
          `}</style>

          <Header>
            <div className="header-top">
              <div className="avatarbox">
                <img width="200" src={thumbnail} />
              </div>
              <div>
                <ul className="social">
                  {me.social.map(({ id, url, icon, text }) => (
                    <li key={id}>
                      <a href={url}>
                        <img alt="" src={icon} height="16" /> {text}
                      </a>
                    </li>
                  ))}
                </ul>
              </div>
            </div>
            <blockquote>{description}</blockquote>
            <Title text="greweb.me" />
          </Header>
          <div className="content">
            <dl>
              <dt>Latest work</dt>
              <div
                style={{
                  display: "flex",
                  flexFlow: "row wrap",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <a href={`/plots/${plots[0].n}`}>
                  <img
                    src={plots[0].data.thumbnail}
                    style={{ width: 300, height: 300, objectFit: "cover" }}
                  />
                </a>
                <a href={`/shaderday/${days[0].n}`}>
                  <Visual width={300} height={300} Day={days[0]} />
                </a>
              </div>

              <dt>
                <Link href="/posts">
                  <a>Latest blog posts</a>
                </Link>
              </dt>

              {posts.slice(0, 3).map((p, i) => (
                <div
                  key={i}
                  style={{
                    display: "flex",
                    flexDirection: "row",
                  }}
                >
                  <img
                    src={p.data.thumbnail}
                    alt=""
                    style={{ width: 150, height: 150, objectFit: "cover" }}
                  />
                  <div style={{ padding: 10 }}>
                    <Link href={`/${p.year}/${p.month}/${p.slug}`}>
                      <a>
                        <strong>{p.data.title}</strong>
                      </a>
                    </Link>
                    <p>{p.data.description}</p>
                  </div>
                </div>
              ))}

              <dt>Plot loops</dt>
              <div
                style={{
                  display: "grid",
                  gridGap: "10px",
                }}
              >
                {plots
                  .filter((p) => p.data.tags?.some((t) => t === "plotloop"))
                  .map((p) => (
                    <a key={p.n} href={`/plots/${p.n}`}>
                      <img
                        src={p.data.thumbnail}
                        style={{
                          width: "100%",
                          height: "100%",
                          objectFit: "cover",
                        }}
                      />
                    </a>
                  ))}
              </div>

              <dt>
                <Link href="https://github.com/gre/gre/tree/master/plots">
                  <a>All plots...</a>
                </Link>
              </dt>
              {plots.map((d) => (
                <dd className="inline" key={d.key}>
                  <Link href={`/plots/${d.n}`}>
                    <a>{String(d.n)}</a>
                  </Link>
                </dd>
              ))}

              <dt>
                <Link href="/shaderday">
                  <a>All shaders...</a>
                </Link>
              </dt>
              {days.map((d) => (
                <dd className="inline" key={d.n}>
                  <Link href={`/shaderday/${d.n}`}>
                    <a>{String(d.n)}</a>
                  </Link>
                </dd>
              ))}

              <dd>
                <Link href="/posts">
                  <a>...more</a>
                </Link>
              </dd>
            </dl>

            <a
              className="twitter-timeline"
              href="https://twitter.com/greweb?ref_src=twsrc%5Etfw"
            ></a>
            <script
              async
              src="https://platform.twitter.com/widgets.js"
              charSet="utf-8"
            ></script>
          </div>
        </Main>
      </Container>
    </Global>
  );
}
