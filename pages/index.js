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
import useDimensions from "react-cool-dimensions";

export async function getStaticProps() {
  const posts = await getAllPosts();
  const plots = await getPlots();
  return {
    props: { posts, plots },
  };
}

const highlightedPlotsIds = [
  "367",
  "577",
  "486",
  "318",
  "331",
  "398",
  "544",
  "527",
  "560",
];

export const PortfolioPlots = ({ plots }) => {
  return (
    <div
      style={{
        display: "grid",
        width: "100%",
        gridTemplateColumns: "1fr 1fr 1fr",
      }}
    >
      {plots
        .filter((p) => highlightedPlotsIds.includes(p.n))
        .sort(
          (a, b) =>
            highlightedPlotsIds.indexOf(a.n) - highlightedPlotsIds.indexOf(b.n)
        )
        .map((plot) => (
          <a href={`/plots/${plot.n}`} key={plot.n} style={{ lineHeight: 0 }}>
            <img
              src={
                plot.data.image && plot.data.image.endsWith(".jpg")
                  ? plot.data.image.replace(/\.([^.]+)$/, "-thumbnail.$1")
                  : plot.data.image
              }
              style={{
                flex: 1,
                width: "100%",
                height: "min(25vw, 25vh)",
                aspectRatio: 1,
                objectFit: "cover",
              }}
            />
          </a>
        ))}
    </div>
  );
};

export const CarouselPlots = ({ plots }) => {
  const [plot, setPlot] = useState(plots[0]);
  const [nonce, setNonce] = useState(0);
  const [prog, setProg] = useState(0);
  useEffect(() => {
    const all = plots.filter((p) => p.data.image);
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
          <a href="/plots">I plot every day...</a>
        </h2>
        <a href={`/plots/${plot.n}`}>
          #{plot.n} – {plot.data.title}
        </a>
      </div>
      <div
        style={{
          background: "#000",
          height: 2,
          margin: "10px 0px 0px 0px",
          boxSizing: "border-box",
          width: (prog * 100).toFixed(2) + "%",
        }}
      />
      <a href={`/plots/${plot.n}`}>
        <img
          src={plot.data.image}
          style={{
            width: "100%",
            height: 440,
            objectFit: "contain",
            border: "10px #000 solid",
            background: "black",
          }}
        />
      </a>
    </>
  );
};

export const StreamIntro = () => {
  return (
    <>
      <h2>
        <a href="https://twitch.tv/greweb">
          I stream, every week... on twitch.tv/greweb
        </a>
      </h2>
      <video
        muted
        loop
        autoPlay
        controls
        src="/images/plots/643-twitch.mp4"
        width="100%"
      ></video>
    </>
  );
};

export const HighlightProjects = ({ projects }) => {
  return null;
};

export const HighlightShader = ({ day }) => {
  const bodyDimensions = useDimensions({});
  const { width, observe } = useDimensions({});

  useEffect(() => {
    bodyDimensions.observe(document.body);
  }, []);

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
          <a href={`/shaderday/${day.n}`}>I also love shaders...</a>
        </h2>
        <a href={`/shaderday/${day.n}`}>
          #{day.n} – {day.title}
        </a>
      </div>
      <a
        ref={observe}
        href={`/shaderday/${day.n}`}
        style={{
          border: "4px solid black",
          boxSizing: "border-box",
          width: "100%",
          position: "relative",
          display: "flex",
          flex: 1,
        }}
      >
        <Visual
          width={Math.min(bodyDimensions?.width - 20, width)}
          height={250}
          Day={day}
        />
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
          <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
          />
          <meta name="og:image" content={`${siteURL}/${thumbnail}`} />
          <meta name="twitter:image" content={`${siteURL}/${thumbnail}`} />
          <link rel="image_src" href={`${siteURL}/${thumbnail}`} />
        </Head>

        <Leva hidden />
        <Main>
          <style jsx>{`
            h1 {
              font-size: 4em;
              font-weight: normal;
              padding: 32px 0;
              margin: 0;
            }
            blockquote {
              max-width: 900px;
              font-weight: 300;
              opacity: 0.5;
              margin: 3em 1em;
            }
            .socials {
              padding: 0px;
              list-style: none;
            }
            .socials a {
              padding: 8px;
              display: inline-block;
              opacity: 0.9;
            }
            .socials a:hover {
              opacity: 1;
            }
            .socials .icon {
              vertical-align: middle;
              height: 28px;
              width: 28px;
              display: inline-block;
            }
            .header-top {
              display: flex;
              flex-direction: row;
              align-items: center;
            }
            .content {
              max-width: 900px;
            }
            section {
              margin-bottom: 50px;
            }
            .subtitle {
              font-size: 1.6em;
              font-weight: 300;
            }
            .subtitle strong {
              font-weight: 800;
            }
            .subtitle a {
              font-weight: 400;
              text-decoration: underline;
            }
            footer {
              text-align: center;
            }
            footer .subtitle {
              font-size: 16px;
            }
            footer .subtitle span:not(:last-child):after {
              content: ", ";
              text-decoration: none;
            }

            .avatarbox img {
              width: 230px;
            }

            @media only screen and (max-width: 600px) {
              .avatarbox img {
                width: 160px;
              }
              h1 {
                font-size: 2em;
              }
              .socials {
                padding-right: 20px;
              }
              .socials img {
              }
              .subtitle {
                font-size: 1em;
              }
            }
          `}</style>

          <Header>
            <div className="header-top">
              <div className="avatarbox">
                <img src={thumbnail} />
              </div>
              <div>
                <h1>@greweb</h1>
                <div className="socials">
                  {me.social.map(({ id, url, icon, text, color, highlighted }) => (
                    <a key={id} href={url} title={text} style={{ color }}>
                      <img className="icon" alt={text} src={icon} />
                      {highlighted ? <strong>{" "}{text}</strong> : null}
                    </a>
                  ))}
                </div>
                <div style={{ textAlign: "right" }}>
                  <img src="/images/mail.jpg" width="300" />
                </div>
              </div>
            </div>
            <blockquote>{description}</blockquote>
            <p className="subtitle">
              I released... <a href="/plots">{plots.length} plots</a>,{" "}
              <a href="/shaderday/1">{days.length} shaders</a>,{" "}
              <a href="/posts">{posts.length} blog posts</a>.
            </p>
          </Header>
          <div className="content">
            <section>
              <PortfolioPlots plots={plots} />
            </section>
            <section>
              <CarouselPlots plots={plots} />
            </section>
            <section>
              <StreamIntro />
            </section>
            <section>
              <HighlightShader day={days.find((d) => d.n === 102)} />
            </section>
            {/*
            <section>
              <h2>
                <Link href="/posts">
                  <a>I sometimes write articles...</a>
                </Link>
              </h2>

              {posts.slice(0, 12).map((p, i) => (
                <Link key={i} href={`/${p.year}/${p.month}/${p.slug}`}>
                  <a>
                    <img
                      src={p.data.thumbnail}
                      alt=""
                      style={{
                        width: "33%",
                        height: "33%",
                        objectFit: "cover",
                      }}
                    />
                  </a>
                </Link>
              ))}
            </section>
                    */}
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
            <footer>
              <p className="subtitle">
                {"@greweb – "}
                {me.social.map(({ id, url, icon, text, extra }) => (
                  <span key={id}>
                    <a href={url}>{id}</a>
                  </span>
                ))}
              </p>
            </footer>
          </div>
        </Main>
      </Container>
    </Global>
  );
}
