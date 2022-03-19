import React, { useEffect, useState } from "react";
import Head from "next/head";
import Link from "next/link";
import sample from "lodash/sample";
import { Leva } from "leva";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { Visual } from "../components/Visual";
import { getDays } from "../shaderdays";
import { getAllPosts } from "../posts";
import { getPlots } from "../plots";
import me from "../me";

const projects2021 = [];

export async function getStaticProps() {
  const posts = await getAllPosts();
  const plots = await getPlots();
  return {
    props: { posts, plots },
  };
}

export const CarouselPlots = ({ plots }) => {
  const [plot, setPlot] = useState(plots[0]);
  const [nonce, setNonce] = useState(0);
  const [prog, setProg] = useState(0);
  useEffect(() => {
    const all = plots.filter((p) => p.data.thumbnail);
    setPlot(nonce < 9 ? all[nonce] : sample(all));
  }, [plots, nonce]);
  useEffect(() => {
    const start = Date.now();
    const delay = 5000;
    const i = setInterval(() => {
      const p = (Date.now() - start) / delay;
      if (p > 1) {
        clearInterval(i);
        setNonce((n) => n + 1);
      } else {
        setProg(p);
      }
    }, 50);
    return () => clearInterval(i);
  }, [plot]);

  return (
    <>
      <style jsx>{`
        h2 {
          padding: 0;
          margin: 0;
        }
      `}</style>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <h2>
          <a href="/plots">Plots</a>
        </h2>
        <a href={`/plots/${plot.n}`}>
          #{plot.n} – {plot.data.title}
        </a>
      </div>
      <div
        style={{
          background: "#000",
          height: 1,
          margin: "4px 0px 0px 0px",
          boxSizing: "border-box",
          width: (prog * 100).toFixed(2) + "%",
        }}
      />
      <a href={`/plots/${plot.n}`}>
        <img
          src={plot.data.thumbnail}
          style={{
            width: "100%",
            height: 600,
            objectFit: "cover",
            border: "4px #000 solid",
          }}
        />
      </a>
    </>
  );
};

export const HighlightProjects = ({ projects }) => {
  return null;
};

export const HighlightShader = ({ day }) => {
  return (
    <>
      <style jsx>{`
        h2 {
          padding: 0;
          margin: 0;
          margin-bottom: 5px;
        }
      `}</style>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <h2>
          <a href="/shaderday">Highlighted Shader</a>
        </h2>
        <a href={`/shaderday/${day.n}`}>
          #{day.n} – {day.title}
        </a>
      </div>
      <a
        href={`/shaderday/${day.n}`}
        style={{
          border: "4px solid black",
          boxSizing: "border-box",
          display: "flex",
        }}
      >
        <Visual width={592} height={300} Day={day} />
      </a>
    </>
  );
};

export default function Home({ posts, plots }) {
  const days = getDays()
    .slice()
    .sort((a, b) => b.n - a.n);

  const siteURL = me.thumbnailDomain;
  const { title, description, thumbnail } = me;

  const minimal =
    typeof location !== "undefined" && location.search === "?minimal";

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
              max-width: 440px;
              font-weight: 300;
              opacity: 0.5;
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
              margin-right: 16px;
            }
            .header-top {
              display: flex;
              flex-direction: row;
              align-items: center;
            }
            .avatarbox {
              padding-right: 20px;
            }
            .content {
              max-width: 600px;
            }
            section {
              margin: 50px 0;
            }
            .subtitle {
              font-size: 24px;
              font-weight: 300;
            }
            .subtitle strong {
              font-weight: 800;
            }
            .subtitle a {
              font-weight: 400;
              text-decoration: underline;
            }
            .minimalfoot strong {
              display: block;
              font-size: 1.2em;
              margin-bottom: 1em;
            }
            .minimalfoot blockquote {
              padding: 0;
              margin: 0;
            }
            .minimal .social {
              width: 340px;
              margin-bottom: 0;
            }
            .minimal .social ul {
              margin: 0;
              padding: 0;
            }
            .minimal .social li {
              display: inline-block;
              font-size: 1.2em;
            }
          `}</style>

          <Header>
            <div className="header-top">
              <div className="avatarbox">
                <img width="230" src={thumbnail} />
              </div>
              <div className={minimal ? "minimal" : ""}>
                {minimal ? (
                  <div className="minimalfoot">
                    <strong>greweb.me</strong>
                    <blockquote>{description}</blockquote>
                  </div>
                ) : null}
                <ul className="social">
                  {me.social.map(({ id, url, icon, text, extra }) =>
                    minimal && extra ? null : (
                      <li key={id}>
                        <a href={url}>
                          <img
                            alt=""
                            src={icon}
                            height={minimal ? "20" : "16"}
                          />{" "}
                          {text}
                        </a>
                      </li>
                    )
                  )}
                </ul>
              </div>
            </div>
            {minimal ? null : (
              <>
                <blockquote>{description}</blockquote>
                <p className="subtitle">
                  <strong>greweb.me</strong> ={" "}
                  <a href="/plots">{plots.length} plots</a>,{" "}
                  <a href="/shaderday">{days.length} shaders</a>,{" "}
                  <a href="/posts">{posts.length} blog posts</a>.
                </p>
              </>
            )}
          </Header>
          {minimal ? null : (
            <div className="content">
              <section>
                <CarouselPlots plots={plots} />
              </section>
              <section>
                <HighlightShader day={days.find((d) => d.n === 102)} />
              </section>
              <section>
                <h2>
                  <Link href="/posts">
                    <a>Recent blog posts</a>
                  </Link>
                </h2>

                {posts.slice(0, 12).map((p, i) => (
                  <Link key={i} href={`/${p.year}/${p.month}/${p.slug}`}>
                    <a>
                      <img
                        src={p.data.thumbnail}
                        alt=""
                        style={{ width: 200, height: 200, objectFit: "cover" }}
                      />
                    </a>
                  </Link>
                ))}
              </section>
              {
                null /*
            <section>
              <h2>2022 projects</h2>
              <HighlightProjects projects={projects2021} />
            </section>
            <section>
              <h2>2021 projects</h2>
              <HighlightProjects projects={projects2021} />
            </section>
              */
              }
            </div>
          )}
        </Main>
      </Container>
    </Global>
  );
}
