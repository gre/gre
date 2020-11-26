import { useState, useEffect } from "react";
import Link from "next/link";

export function Title() {
  return (
    <>
      <h1 className="title">
        <Link href="/">
          <a>One Day, One Shader</a>
        </Link>
      </h1>

      <style jsx>{`
        .title {
          text-align: center;
          margin: 0;
          line-height: 1.15;
          font-size: 2rem;
        }
      `}</style>
    </>
  );
}
