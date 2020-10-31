import { useState, useEffect } from "react";
import Head from "next/head";
import Link from "next/link";
import { Surface } from "gl-react-dom";

const Refresh = ({ Day }) => {
  const [time, setTime] = useState(0);
  useEffect(() => {
    let startT;
    function loop(t) {
      requestAnimationFrame(loop);
      if (!startT) startT = t;
      setTime((t - startT) / 1000);
    }
    requestAnimationFrame(loop);
  }, []);
  return <Day.Shader time={time} />;
};

export function Page({ Day }) {
  return (
    <div className="container">
      <Head>
        <title>One Day One Shader</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <header>
          <h1 className="title">
            <Link href="/">
              <a>One Day, One Shader</a>
            </Link>
          </h1>

          <p className="description">
            <Link href={`/day/${Day.n}`}>
              <a>Day {Day.n}.</a>
            </Link>{" "}
            <span className="daily">"{Day.title}"</span> by{" "}
            <a href="https://twitter.com/greweb" target="_blank">
              @greweb
            </a>
          </p>
        </header>

        <div className="day">
          <Surface width={400} height={400}>
            <Refresh Day={Day} />
          </Surface>
        </div>

        <footer>
          <a
            target="_blank"
            href={`https://github.com/gre/one-day-one-shader/blob/master/day/${(
              Day.n + ""
            ).padStart(3, "0")}.js`}
          >
            source code
          </a>
        </footer>
      </main>

      <style jsx>{`
        .container {
          min-height: 100vh;
          padding: 0 0.5rem;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }

        main {
          flex: 1;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }

        a {
          color: inherit;
          text-decoration: none;
        }

        a:hover,
        a:active {
          text-decoration: underline;
        }

        .title,
        .description {
          text-align: center;
        }

        .title {
          margin: 0;
          line-height: 1.15;
          font-size: 2rem;
        }

        .description {
          line-height: 1rem;
          font-size: 1.5rem;
          font-weight: 300;
        }

        .description .daily {
          font-weight: 600;
        }

        header {
          padding: 1rem 0;
        }
        footer {
          padding: 1rem 0;
          font-size: 0.8rem;
          font-style: italic;
        }
      `}</style>

      <style jsx global>{`
        html,
        body {
          padding: 0;
          margin: 0;
          font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto,
            Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue,
            sans-serif;
        }

        * {
          box-sizing: border-box;
        }
      `}</style>
    </div>
  );
}
