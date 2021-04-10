import React from "react";
import Head from "next/head";
import Link from "next/link";
import { Title } from "../components/Title";
import { Container } from "../components/Container";
import { Global } from "../components/Global";
import { Main } from "../components/Main";
import { Header } from "../components/Header";
import { getDays } from "../shaderdays";
import { getAllPosts } from "../posts";
import { getPlots } from "../plots";

export async function getStaticProps() {
  const posts = await getAllPosts();
  const plots = await getPlots();
  return {
    props: { posts, plots },
  };
}

export default function Home({ posts, plots }) {
  return (
    <Global>
      <Container>
        <Head>
          <title>greweb.me</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>
        <Main>
          <style jsx>{`
            blockquote {
              max-width: 500px;
            }
            .content {
              max-width: 600px;
              padding: 0 10px;
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
          `}</style>

          <Header>
            <img width="200" src="/profile.jpg" />
            <Title text="greweb.me" />
            <p className="social">
              <a href="https://twitter.com/greweb">Twitter</a>
              {" – "}
              <a href="https://github.com/gre">Github</a>
              {" – "}
              <a href="https://twitch.tv/greweb">Twitch</a>
            </p>
            <blockquote>
              Gaëtan Renaudeau (greweb). French developer at Ledger. creative
              coder experimenting with GLSL shaders, Rust and fountain pens
              robot plots. infinite noise explorer. learn Chinese.
            </blockquote>
            <div
              style={{
                background: "black",
                color: "white",
                padding: "0.2em 0.5em",
                marginBottom: 10,
                fontSize: "0.8em",
              }}
            >
              ⚠ greweb.me under construction!
            </div>
          </Header>

          <div className="content">
            <dl>
              <dt>
                <Link href="/shaderday">
                  <a>Shaders</a>
                </Link>
              </dt>
              {getDays()
                .slice()
                .sort((a, b) => b.n - a.n)
                .map((d) => (
                  <dd className="inline" key={d.n}>
                    <Link href={`/shaderday/${d.n}`}>
                      <a>{String(d.n)}</a>
                    </Link>
                  </dd>
                ))}

              <dt>
                <Link href="https://github.com/gre/gre/tree/master/plots">
                  <a>Plots</a>
                </Link>
              </dt>
              {plots.map((d) => (
                <dd className="inline" key={d.key}>
                  <Link
                    href={`https://github.com/gre/gre/tree/master/plots/examples/${d.rustFile}`}
                  >
                    <a>{String(d.n)}</a>
                  </Link>
                </dd>
              ))}

              <dt>
                <Link href="/posts">
                  <a>Latest blog posts</a>
                </Link>
              </dt>
              {posts.slice(0, 3).map((p, i) => (
                <dd key={i}>
                  <Link href={`/${p.year}/${p.month}/${p.slug}`}>
                    <a>{p.data.title}</a>
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
              charset="utf-8"
            ></script>
          </div>
        </Main>
      </Container>
    </Global>
  );
}
