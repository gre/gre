import { useState, useEffect } from "react";
import Link from "next/link";
import { findDay } from "../shaderdays";

export function SubTitle({ Day }) {
  return (
    <>
      <p className="description">
        <Link href={`/shaderday/${Day.n}`}>
          <a>Day {Day.n}.</a>
        </Link>{" "}
        <span className="daily">"{Day.title}"</span> by{" "}
        <a href="https://twitter.com/greweb" target="_blank">
          @greweb
        </a>
      </p>
      <nav>
        {findDay(Day.n - 1) ? (
          <Link href={`/shaderday/${Day.n - 1}`}>
            <a>← previous day</a>
          </Link>
        ) : (
          <span />
        )}
        {findDay(Day.n + 1) ? (
          <Link href={`/shaderday/${Day.n + 1}`}>
            <a>next day →</a>
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
          width: 400px;
          font-size: 0.6em;
          display: flex;
          flex-direction: row;
          justify-content: space-between;
        }
      `}</style>
    </>
  );
}
