import React, { useEffect, useRef, useState } from "react";
import regl from "regl";
import Head from "next/head";
import Link from "next/link";
import { Global } from "../../components/Global";
import { Container } from "../../components/Container";
import { Main } from "../../components/Main";
import { Content } from "../../components/Content";
import { Header } from "../../components/Header";
import { Title } from "../../components/Title";
import { art, generate } from "../../doodles/generative-nano-s-plus/dist/main";

function Render({ index, width, height }) {
  const ref = useRef();
  useEffect(() => {
    const { opts } = generate(index);
    const c = regl(ref.current);
    const frameTime = (_, o) => o.time;
    const onFrame = () => {};
    const createCanvas = (w, h) => {
      const canvas = document.createElement("canvas");
      canvas.width = w;
      canvas.height = h;
      return canvas;
    };
    const antialias = false;
    art(
      c,
      opts,
      frameTime,
      onFrame,
      createCanvas,
      (ctx) =>
        (...args) =>
          ctx.fillText(...args),
      (canvas) => ({ data: canvas, flipY: true }),
      false,
      antialias,
      0.25
    );
    return () => c.destroy();
  }, [index]);
  const dpr =
    (typeof window !== "undefined" ? window.devicePixelRatio : null) || 1;
  return (
    <canvas
      ref={ref}
      width={Math.round(width * dpr)}
      height={Math.round(height * dpr)}
      style={{
        width,
        height,
      }}
    ></canvas>
  );
}

export default function Home() {
  const title = "GNSP Collection";
  const [index, setIndex] = useState(() => Math.floor(2048 * Math.random()));
  useEffect(() => {
    const i = setInterval(() => {
      setIndex((i) => (i + 1) % 2048);
    }, 4000);
    return () => clearInterval(i);
  }, []);
  const { metadata } = generate(index);

  return (
    <Global>
      <Head>
        <title>greweb.me – {title}</title>
        <link rel="icon" href="/favicon.ico" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="author" content="Gaëtan Renaudeau" />
        <meta name="keywords" content={"nft, collection"} />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:site" content="@greweb" />
        <meta name="twitter:title" content={title} />
        <meta name="twitter:creator" content="@greweb" />
        <meta
          name="twitter:image"
          content="http://greweb.me/images/gnsp/cover.jpg"
        />
        <link rel="image_src" href="http://greweb.me/images/gnsp/cover.jpg" />
        <meta
          property="og:image"
          content="http://greweb.me/images/gnsp/cover.jpg"
        />
        <base target="_blank" />
      </Head>
      <Container>
        <Main>
          <Header>
            <Title withBreadcrumb text={title} />
          </Header>

          <style jsx>{`
            .cta {
              background: #f0f;
              color: #fff;
              display: inline-block;
              margin: 0.4em 0;
              padding: 0.4em 0.8em;
              font-size: 32px;
              cursor: pointer;
            }
            .cta:hover {
              text-decoration: none;
              opacity: 0.8;
            }
          `}</style>

          <Content>
            <p>
              <Link href={"/gnsp/" + index}>
                <a>
                  <strong>{metadata.name}</strong> ({index + 1} / 2048)
                </a>
              </Link>
            </p>
            <Render index={index} width={640} height={640} />
            <p style={{ whiteSpace: "pre-wrap" }}>{metadata.description}</p>

            <h2>More content will be shared later</h2>

            <footer
              style={{
                textAlign: "center",
                fontStyle: "italic",
                fontSize: "20px",
                margin: "20px 0",
                padding: "4px 16px",
                background: "#000",
                color: "white",
              }}
            >
              <p>
                @greweb has been doing generative art for many years, shaders,
                and more recently fountain pens robot plotting!
              </p>

              <p>
                His work is about exploring the beauty of noise through many
                algorithms. See also{" "}
                <Link href="/plots">
                  <a>https://greweb.me/plots</a>
                </Link>
              </p>

              <img width="50%" src="/images/2021/08/artist-1.jpg" />
              <img width="50%" src="/images/2021/08/artist-2.jpg" />
            </footer>
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
