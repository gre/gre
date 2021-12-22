import React, { useEffect, useState } from "react";
import Head from "next/head";
import Link from "next/link";
import { Global } from "../../components/Global";
import { Container } from "../../components/Container";
import { Main } from "../../components/Main";
import { Content } from "../../components/Content";
import { Header } from "../../components/Header";
import { Title } from "../../components/Title";
import { generate } from "../../doodles/generative-nano-s-plus/dist/main";

const videoMode = true;

function Cell({ index, width, height, delay }) {
  const { metadata } = generate(index);
  const [visible, setVisible] = useState(delay < 1);
  useEffect(() => {
    const t = setTimeout(() => setVisible(true), delay);
    return () => clearTimeout(t);
  }, [delay]);
  return (
    <Link href={"/gnsp/" + index}>
      <a>
        {visible ? (
          videoMode ? (
            <video
              controls
              muted
              autoPlay
              loop
              src={metadata.animation_url}
              width={width}
              height={height}
            />
          ) : (
            <img
              width={width}
              height={height}
              alt={"" + index}
              title={"" + index}
              src={metadata.image}
            />
          )
        ) : (
          <div style={{ width, height }} />
        )}
      </a>
    </Link>
  );
}

export default function Home() {
  const title = "GNSP Tools";
  const [req, setReq] = useState(null);
  const [indexes, setIndexes] = useState([]);
  useEffect(() => {
    const t = setTimeout(() => {
      try {
        const predicate = new Function(
          "features,opts",
          "return (" + (req || "true") + ")"
        );
        setIndexes(
          Array(2048)
            .fill(null)
            .map((_, i) => i)
            .sort(() => Math.random() - 0.5)
            .filter((i) => {
              const { metadata, opts } = generate(i);
              const features = {};
              metadata.attributes.forEach(({ trait_type, value }) => {
                features[trait_type] = value;
              });
              return predicate(features, opts);
            })
        );
      } catch (e) {
        console.log(e);
      }
    }, 200);
    return () => clearTimeout(t);
  }, [req]);

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

          <div style={{ padding: "30px", width: "100vw" }}>
            <div>
              <input
                type="text"
                value={req}
                onChange={(e) => setReq(e.target.value)}
                style={{
                  fontSize: "20px",
                  padding: "0.8em",
                  width: "100%",
                }}
              />
            </div>
            <p>{indexes.length} results.</p>
            {indexes.slice(0, 8).map((index, i) => (
              <Cell
                key={index}
                index={index}
                width={325}
                height={325}
                delay={Math.pow(i, 1.5) * 1000}
              />
            ))}
          </div>
        </Main>
      </Container>
    </Global>
  );
}
