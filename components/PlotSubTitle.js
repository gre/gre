import React from "react";
import Link from "next/link";

export function SubTitle({ plot, prev, next }) {
  return (
    <>
      <p className="description">
        <Link href={`/plots/${plot.n}`}>
          <a>Plot #{plot.n}</a>
        </Link>{" "}
        {plot.data.title ? (
          <span className="daily">&quot;{plot.data.title}&quot;</span>
        ) : null}{" "}
        by{" "}
        <a href="https://twitter.com/greweb" target="_blank" rel="noreferrer">
          @greweb
        </a>
      </p>
      <nav>
        {prev ? (
          <Link href={`/plots/${prev.n}`}>
            <a>← previous plot</a>
          </Link>
        ) : (
          <span />
        )}
        {next ? (
          <Link href={`/plots/${next.n}`}>
            <a>next plot →</a>
          </Link>
        ) : null}
      </nav>
      <style jsx>{`
        .description {
          text-align: center;
          line-height: 1.3rem;
          font-size: 1.3rem;
          font-weight: 300;
        }

        .description .daily {
          font-weight: 600;
        }

        nav {
          width: 100%;
          font-size: 0.6em;
          display: flex;
          flex-direction: row;
          justify-content: space-between;
          padding: 10px;
        }
      `}</style>
    </>
  );
}
