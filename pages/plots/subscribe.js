import React, { useEffect, useState } from "react";
import Head from "next/head";
import Link from "next/link";
import { Global } from "../../components/Global";
import { Container } from "../../components/Container";
import { Main } from "../../components/Main";
import { Content } from "../../components/Content";
import { Header } from "../../components/Header";
import { Title } from "../../components/Title";

const lock = "0x1B18334Df6d036325151D2b5d695Ef7c330CeC90";

export default function Home({ tag }) {
  const title = `Subscribe to monthly plots`;

  const [unlockStatus, setUnlockStatus] = useState(null);

  useEffect(() => {
    if (typeof document === "undefined") return;
    return (function (d, s) {
      window.unlockProtocolConfig = {
        network: 4,
        locks: {
          [lock]: {
            name: "Greweb Monthly Plot",
          },
        },
        icon: "/images/plots-promo/letters.jpeg",
        callToAction: {
          default:
            "Purchase a membership to get physical art shipped internationally. Come back to renew every month.",
          metadata:
            "Shipping information is needed to get physical art shipped internationally. Feel free to use a PO Box / Poste Restante service. A tracked letter will be used and shared to you by email.",
        },
        metadataInputs: [
          {
            name: "Full postal address (with name and country)",
            type: "text",
            required: true,
          },
          {
            name: "Email (to get the tracked number)",
            type: "email",
            required: true,
          },
        ],
      };

      var js = d.createElement(s),
        sc = d.getElementsByTagName(s)[0];
      js.src =
        "https://paywall.unlock-protocol.com/static/unlock.latest.min.js";
      sc.parentNode.insertBefore(js, sc);

      const t = setTimeout(() => {
        setUnlockStatus({ state: "notLoaded" });
      }, 10000);

      window.addEventListener("unlockProtocol.status", (e) => {
        clearTimeout(t);
        setUnlockStatus(e.detail);
      });

      return () => {
        clearTimeout(t);
      };
    })(document, "script");
  }, []);

  const onPurchase = () => {
    window.unlockProtocol.loadCheckoutModal();
  };

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
      </Head>
      <div
        style={{
          fontWeight: "bold",
          padding: 5,
          background: "#F0F",
          color: "white",
          textAlign: "center",
          position: "sticky",
          top: 0,
          width: "100%",
        }}
      >
        BETA: this offer is currently in BETA TEST and is done on Ethereum
        Rinkeby at the moment.
      </div>
      <Container>
        <Main>
          <Header>
            <Title withBreadcrumb text={title} />
          </Header>

          <style jsx>{`
            .cta {
              background: #000;
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
            <h3>
              Every month, @greweb is shipping original plots worldwide to
              subscribers of this NFT membership, renewable each month.
            </h3>
            <p>
              <strong>On 1th of the month</strong>, a unique generator is
              developed by @greweb specially for "Greweb Monthly Plot"
              collectors. Every member will get a random original art piece from
              the generator. It is physically plotted with an AxiDraw by the
              artist.
            </p>
            <p>
              <strong>On 15th of the month</strong>, a snapshot of the
              membership is captured. The physical art originals are plotted and
              shipped for each member, using a tracked letter. Don't forget to
              renew your membership before this date!
            </p>
            <div
              style={{ background: "#eee", padding: 10, textAlign: "center" }}
            >
              {unlockStatus ? (
                unlockStatus.state === "unlocked" ? (
                  <strong>Your membership is active!</strong>
                ) : (
                  <div>
                    <p>
                      Purchase membership of a "Greweb Monthly Plot" NFT to get
                      physical art originals from @greweb. Shipping details are
                      filled with the Unlock purchase.
                    </p>
                    <p>No membership found (or expired/logged out)</p>
                    <a className="cta" onClick={onPurchase}>
                      Purchase membership
                    </a>
                  </div>
                )
              ) : (
                <p>Loading...</p>
              )}
              <p style={{ fontStyle: "italic", fontSize: "0.8em" }}>
                {"Manage your memberships on "}
                <a href="https://app.unlock-protocol.com/keychain">
                  https://app.unlock-protocol.com/keychain
                </a>
              </p>
            </div>
            <img width="100%" src="/images/plots-promo/letters.jpeg" />
            <div style={{ padding: 20, background: "#EEE" }}>
              <h3>Next Month Preview</h3>
              Here is a preview of what is planned for December 2021 art.
              <br />
              [PREVIEW IMAGE WOULD BE HERE. TODO DYNAMIC CONTENT]
            </div>

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
                @greweb has been doing generative art for many years, shaders
                and more recently fountain pens robot plotting!
              </p>

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

              <p>
                His work is about exploring beauty of noise through many
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
