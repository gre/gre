import React, { useEffect, useMemo, useRef, useState } from "react";
import regl from "regl";
import Head from "next/head";
import Link from "next/link";
import { Global } from "../../../components/Global";
import { Container } from "../../../components/Container";
import { Main } from "../../../components/Main";
import { Content } from "../../../components/Content";
import { Header } from "../../../components/Header";
import { Title } from "../../../components/Title";
import {
  art,
  generate,
} from "../../../doodles/generative-nano-s-plus/dist/main";
import useDimensions from "react-cool-dimensions";

const all = Array(2048)
  .fill(null)
  .map((_, i) => i);

export async function getStaticPaths() {
  const paths = all.map((index) => ({
    params: { index: String(index) },
  }));
  return {
    paths,
    fallback: false,
  };
}

export async function getStaticProps({ params }) {
  const index = parseInt(params.index, 10);
  if (index < 0 || index >= 2048) {
    throw new Error("invalid");
  }
  return {
    props: { index },
  };
}

function Render({ index, width, height }) {
  const ref = useRef();
  const { opts, metadata } = useMemo(() => generate(index), [index]);
  useEffect(() => {
    console.log(opts);
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
  }, [opts, index]);
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

export default function Home({ index }) {
  const { metadata, opts } = generate(index);
  const [width, setWidth] = useState(() =>
    typeof window === "undefined" ? 0 : window.innerWidth
  );
  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }
    function onResize() {
      setWidth(window.innerWidth);
    }
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, []);
  const sz = Math.max(120, Math.min(width - 40, 400));

  const title = metadata.name;

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
            <h1
              style={{
                textAlign: "center",
                margin: "0.4em 0",
                fontSize: "1.5rem",
              }}
            >
              <Link href="/">
                <a>greweb.me</a>
              </Link>
              {" / "}
              <Link href="/gnsp">
                <a>GNSP</a>
              </Link>
              {" / "}'{opts.word}'
            </h1>
          </Header>

          <nav
            style={{
              display: "flex",
              flexDirection: "row",
              alignItems: "center",
              justifyContent: "space-between",
              width: "100%",
            }}
          >
            {index <= 0 ? (
              <span />
            ) : (
              <Link href={"/gnsp/" + (index - 1)}>
                <a>prev</a>
              </Link>
            )}
            <span>{index + 1} out of 2048</span>
            {index >= 2047 ? (
              <span />
            ) : (
              <Link href={"/gnsp/" + (index + 1)}>
                <a>next</a>
              </Link>
            )}
          </nav>

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
            <p>{metadata.description}</p>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
              }}
            >
              {typeof window === "undefined" ? null : (
                <div
                  style={{
                    padding: 40,
                    backgroundColor: metadata.background_color,
                    display: "flex",
                  }}
                >
                  <Render index={index} width={sz} height={sz} />
                </div>
              )}

              <footer style={{ padding: "20px 0" }}>
                <Link href="/gnsp">
                  <a>See homepage explanation of GNSP</a>
                </Link>
              </footer>

              {/*
              <footer>
                <pre>
                  <code>
                    {metadata.attributes
                      .map(({ trait_type, value }) => trait_type + ": " + value)
                      .join("\n")}
                  </code>
                </pre>
              </footer>
              */}
            </div>
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
