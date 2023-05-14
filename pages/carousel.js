import React, { useEffect, useState } from "react";
import Head from "next/head";
import sample from "lodash/sample";
import { Global } from "../components/Global";
import { getAllPosts } from "../posts";
import { getPlots } from "../plots";

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
    const all = plots.filter((p) => p.data.image);
    const index = Math.floor(Math.random() * Math.random() * all.length);
    setPlot(all[index]);
  }, [plots, nonce]);
  useEffect(() => {
    const start = Date.now();
    const delay = 10000;
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
        <strong>{plot.data.title}</strong>
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
      <img
        src={plot.data.image}
        style={{
          width: "100%",
          height: "80vh",
          objectFit: "contain",
          border: "10px #000 solid",
          background: "black",
        }}
      />

      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          alignItems: "center",
          marginTop: 6,
        }}
      >
        <em>{plot.data.date}</em>{" – "}<span style={{ textDecoration: "underline" }}>greweb.me/plots/{plot.n}</span>
      </div>
    </>
  );
};

export default function Home({ plots }) {
  return (
    <Global>

      <div className="container">

        <Head>
          <title>Carousel</title>
          <link rel="icon" href="/favicon.ico" />
          <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
          />
        </Head>
        <CarouselPlots plots={plots} />

        <style jsx>{`
        .container {
          min-height: 100vh;
          padding: 0;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }
      `}</style>
      </div>
    </Global>
  );
}
