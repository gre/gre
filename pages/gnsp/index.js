import React, { useEffect, useMemo, useRef, useState } from "react";
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
  const { opts, metadata } = useMemo(() => generate(index), [index]);
  useEffect(() => {
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
    return () => {
      c.destroy();
    };
  }, [index, opts]);
  const dpr =
    (typeof window !== "undefined" ? window.devicePixelRatio : null) || 1;
  return (
    <div
      style={{
        alignItems: "center",
        display: "flex",
        flexDirection: "column",
        padding: 10,
      }}
    >
      <div
        style={{
          position: "relative",
          width,
          height,
          boxSizing: "content-box",
          transition: "1s border",
          border: "40px solid " + metadata.background_color,
        }}
      >
        <Link href={"/gnsp/" + index}>
          <a target="_blank">
            <p
              style={{
                position: "absolute",
                bottom: -20,
                right: 0,
                margin: 0,
                width: "100%",
                textAlign: "center",
                color: "#444",
                fontWeight: 300,
              }}
            >
              {metadata.name} ({index + 1} / 2048)
            </p>
            <canvas
              ref={ref}
              width={Math.round(width * dpr)}
              height={Math.round(height * dpr)}
              style={{ width, height }}
            ></canvas>
          </a>
        </Link>
      </div>
    </div>
  );
}

export default function Home() {
  const title = "GNSP Collection";

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
  const sz = Math.max(120, Math.min(width - 80, 360));

  const [index, setIndex] = useState(() => Math.floor(2048 * Math.random()));
  useEffect(() => {
    const i = setInterval(() => {
      setIndex((i) => (i + 1) % 2048);
    }, 8000);
    return () => clearInterval(i);
  }, []);
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
            {typeof window === "undefined" ? null : (
              <Render index={index} width={sz} height={sz} />
            )}

            <p>
              2048 items NFT collection, one per unique word in the BIP39
              wordlist. GNSP is short for Generative Nano S+ – Ledger's new
              hardware wallet – which this collection is celebrating. There are
              rarity aspects in the colors, background, animations and swivel
              engraved content.
            </p>
            <p>
              Using a hardware wallet is important to secure your crypto and
              secure your NFTs and making a NFT collection about it was for me a
              way to share this importance.
            </p>

            <h2>Distribution</h2>

            <p>
              The NFTs are available on Polygon blockchain and has been
              initially distributed as Christmas gift to Ledger people. The rest
              of the collection is going to be released in a second phase on
              this website. Stay tuned!{" "}
              <a href="https://twitter.com/greweb">(@greweb)</a>
            </p>

            <p>
              The collection is available on
              <a href="https://opensea.io/collection/gnsp">OpenSea</a> but not
              everything has been minted yet!
            </p>

            <h2>More content will be shared later</h2>

            <p>
              This was a great technical journey to work on this collection.
              Source code will be shared as well as many technical challenges of
              this generative art work.
            </p>

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
