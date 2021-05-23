import React from "react";
import Link from "next/link";

export function Title({ withBreadcrumb, text }) {
  return (
    <>
      <h1 className="title">
        {withBreadcrumb ? (
          <Link href="/">
            <a>greweb.me</a>
          </Link>
        ) : null}
        {withBreadcrumb ? " / " : ""}
        {text}
      </h1>

      <style jsx>{`
        .title {
          text-align: center;
          margin: 0.5em 0;
          font-size: 1.5rem;
        }
      `}</style>
    </>
  );
}
