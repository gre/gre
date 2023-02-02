import React, { useEffect, useState } from "react";
import Head from "next/head";
import Link from "next/link";
import { Global } from "../../components/Global";
import { Container } from "../../components/Container";
import { Main } from "../../components/Main";
import { Content } from "../../components/Content";
import { Header } from "../../components/Header";
import { Title } from "../../components/Title";

export function PlottingSectionVideos() {
  return (
    <>
      <video
        loop
        autoPlay
        muted
        src="/images/plots/164-plotting-speed-x200.mp4"
        width="50%"
      ></video>
      <video
        loop
        autoPlay
        muted
        src="/images/plots/164-showcase.mp4"
        width="50%"
      ></video>
    </>
  );
}

export function PlottingFooter() {
  return (
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
        @greweb loves exploring the beauty of noise through many algorithms,
        notably using shaders and plotters. See also{" "}
        <Link href="/plots">
          <a>https://greweb.me/plots</a>
        </Link>
      </p>

      <img width="50%" src="/images/2021/08/artist-1.jpg" />
      <img width="50%" src="/images/2021/08/artist-2.jpg" />
    </footer>
  );
}

const Row = ({ children }) => (
  <div style={{ padding: "0.5em 0" }}>{children}</div>
);

const Choice = ({ children }) => (
  <div
    style={{
      display: "flex",
      flexDirection: "row",
      alignItems: "center",
      gap: 8,
    }}
  >
    {children}
  </div>
);

const Circle = ({ n }) => (
  <>
    <style jsx>{`
      .circle {
        display: inline-block;
        border: 2px #f0f solid;
        padding: 0.2em 0.4em;
        margin-right: 0.5em;
        color: #f0f;
      }
    `}</style>
    <span className="circle">{n}</span>
  </>
);

const Price = ({ children }) => (
  <>
    <style jsx>{`
      .price {
        display: inline-block;
        padding: 0.5em;
        font-size: 1.25em;
        background: #fff;
        border: 2px solid #f0f;
        color: #f0f;
        font-weight: bold;
        min-width: 120px;
        text-align: center;
      }
    `}</style>
    <span className="price">{children}</span>
  </>
);

const Token = ({ children, url }) => (
  <>
    <style jsx>{`
      a {
        display: inline-block;
        padding: 0.5em;
        font-size: 0.8em;
        font-weight: 400;
        background: #fff;
        border: 2px solid #f0f;
        color: #f0f;
        max-width: 120px;
        text-align: center;
      }
    `}</style>
    <a target="_blank" href={url}>
      <strong>1</strong> Greweb Plot Request Token
    </a>
  </>
);

const Address = ({ children, real }) => (
  <>
    <style jsx>{`
      .address {
        display: inline-block;
        padding: 0.1em 0.5em;
        border: 1px solid #f0f;
        color: #f0f;
        display: inline-flex;
        align-items: center;
        flex-direction: column;
        min-width: 200px;
        text-align: center;
      }
      .main {
        font-size: 1.2em;
        user-select: all;
        padding: 0.2em;
      }
      .sub {
        display: inline-block;
        font-size: 0.4em;
        color: #f0f;
        user-select: all;
        padding: 0.2em;
      }
    `}</style>
    <span>
      <span className="address">
        <span className="main">{children}</span>
        <span className="sub">{real}</span>
      </span>
    </span>
  </>
);

const choices = [
  {
    id: "ethereum",
    name: "Ethereum",
    address: "greweb.eth",
    addressReal: "0x68db7D679969f265b14BA8A495E4028360AD6759",
    amount: "0.04 ETH",
    collections: [
      {
        url: "https://opensea.io/collection/blockart?search[sortAscending]=true&search[sortBy]=PRICE&search[stringTraits][0][name]=Style&search[stringTraits][0][values][0]=Pattern%2003",
        description: "Pattern03 via ethblock.art on OpenSea (888 mints)",
      },
      {
        url: "https://opensea.io/collection/plottables",
        description:
          "A plottable collection – IF the generator artists allows it",
      },
    ],
  },
  {
    id: "tezos",
    name: "Tezos",
    address: "greweb.tez",
    addressReal: "tz1cgQAQfECg5bPASYTMyJ9QJQjSUi8rfL67",
    amount: "25 tz",
    tokenNftURL:
      "https://objkt.com/asset/KT1EhesmoVKQ3qTjG9V2MmYxQn7HVtozk3RP/0",
    collections: [
      {
        url: "https://www.fxhash.xyz/generative/24533",
        description: "Plottable Sliced Spiral is FREE if you are the initial minter (fill the form without sending)"
      },
      {
        url: "https://www.fxhash.xyz/u/greweb",
        description: "Any Plottable work from fxhash.xyz/u/greweb",
      },
    ],
  },
  {
    id: "cosmos",
    name: "Cosmos",
    address: "cosmos15rce70qlpcztvvekjwpv4fx3s5k2ujjed3vfce",
    addressReal: "cosmos15rce70qlpcztvvekjwpv4fx3s5k2ujjed3vfce",
    amount: "2 atom",
    collections: [
      {
        url: "https://publicworks.art",
        description: "Any Plottable work from https://publicworks.art",
      },
    ],
  },
];

const CTA = ({ children, ...rest }) => (
  <>
    <style jsx>{`
      .cta {
        display: inline-block;
        padding: 0.4em 0.8em;
        font-size: 1.2em;
        background: #f0f;
        color: #fff;
        cursor: pointer;
      }
      .cta:hover {
        opacity: 0.8;
      }
    `}</style>
    <a className="cta" {...rest}>
      {children}
    </a>
  </>
);

const Or = () => {
  const sep = {
    width: "100%",
    height: "2px",
    background: "#F0F",
  };
  return (
    <div
      style={{
        maxWidth: "350px",
        display: "flex",
        flexDirection: "row",
        fontSize: "1.4em",
        fontWeight: "bold",
        alignItems: "center",
        margin: "20px 0",
      }}
    >
      <div style={sep}></div>
      OR
      <div style={sep}></div>
    </div>
  );
};

export default function Home({ tag }) {
  const title = `Getting physical plot of a Plottable NFT`;

  const [i, setI] = useState(1);
  const choice = choices[i];

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
        <meta
          name="twitter:image"
          content="http://greweb.me/images/plots-promo/letters.jpeg"
        />
        <link
          rel="image_src"
          href="http://greweb.me/images/plots-promo/letters.jpeg"
        />
        <meta
          property="og:image"
          content="http://greweb.me/images/plots-promo/letters.jpeg"
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
            <div
              style={{
                padding: "10px 0",
                margin: "40px 0",
                borderTop: "4px solid #F0F",
                borderBottom: "4px solid #F0F",
                color: "#F0F",
              }}
            >
              <h3 style={{ padding: 0, marginTop: 0 }}>
                Request your physical plot from @greweb
              </h3>

              <Row>
                <Circle n={1} />
                <strong>NFT ownership</strong>: Make sure you own the digital
                Plottable NFT
                <div
                  style={{
                    padding: "0.8em 3em",
                  }}
                >
                  <select
                    style={{
                      fontSize: "1.2em",
                      color: "#FFF",
                      background: "#F0F",
                      padding: "8px",
                      borderRadius: "0",
                      border: "0",
                    }}
                    onChange={(e) => setI(e.target.selectedIndex)}
                    value={i}
                  >
                    {choices.map(({ id, name }, i) => (
                      <option key={id} value={i}>
                        on {name}
                      </option>
                    ))}
                  </select>
                  {choice.collections ? (
                    <div style={{ opacity: 0.6 }}>
                      <ul
                        style={{
                          padding: 0,
                          margin: "1em 0 0 20px",
                        }}
                      >
                        {choice.collections.map((col) => (
                          <li key={col.url}>
                            <a href={col.url}>{col.description}</a>
                          </li>
                        ))}
                      </ul>
                    </div>
                  ) : null}
                </div>
              </Row>
              <Row>
                <Circle n={2} />
                <strong>Physical cost</strong>: Send from the{" "}
                <span style={{ textDecoration: "underline" }}>
                  same address owning the Plottable NFT
                </span>
                <div
                  style={{
                    marginTop: "0.8em",
                    fontWeight: 200,
                    padding: "0 0 0 3em",
                    display: "flex",
                    flexDirection: "column",
                  }}
                >
                  <Choice>
                    <Price>{choice.amount}</Price> to{" "}
                    <Address real={choice.addressReal}>
                      {choice.address}
                    </Address>
                  </Choice>
                  {choice.tokenNftURL ? (
                    <>
                      <Or />
                      <Choice>
                        <Token url={choice.tokenNftURL} /> to{" "}
                        <Address real={choice.addressReal}>
                          {choice.address}
                        </Address>
                      </Choice>
                    </>
                  ) : null}
                </div>
              </Row>
              <Row>
                <Circle n={3} />
                Provide the shipping address and proof of transaction:
                <div style={{ padding: "0em 3em" }}>
                  <p>
                    <CTA href="https://forms.gle/JWUfuAjochGQ9BQu7">
                      Fill this Google Form
                    </CTA>
                  </p>
                </div>
              </Row>
            </div>

            <h3
              style={{
                fontWeight: 200,
                fontSize: "1.6em",
                marginTop: 0,
                paddingTop: 10,
              }}
            >
              @greweb is happy to physically plot art on demand and deliver
              signed plots worldwide to "plottable NFT" digital collectors.
            </h3>

            <img width="100%" src="/images/plots-promo/letters.jpeg" />

            <div
              style={{
                padding: "0 40px",
                margin: 0,
              }}
            >
              <p>
                In this paradigm, the digital NFT is decoupled from their
                possible physical counterpart. While the NFT live on its own
                form, owning it gives you the power to claim here a physical
                counterpart (even obtained on second market). A physical price
                (in digital currency) is asked to cover physical plotting and
                delivery cost. Price is set on this page and may evolve in
                future (but it is the same for all NFTs) A physical plot can be
                produced multiple times but every re-plot are unique due to this
                analog process.
              </p>
            </div>

            <PlottingSectionVideos />
            <PlottingFooter />
          </Content>
        </Main>
      </Container>
    </Global>
  );
}
