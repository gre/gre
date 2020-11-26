import { useState, useEffect } from "react";

export function Global({ children }) {
  return (
    <>
      {children}

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
        a {
          color: inherit;
          text-decoration: none;
        }

        a:hover,
        a:active {
          text-decoration: underline;
        }
      `}</style>
    </>
  );
}
