import React from "react";
import HttpsRedirect from "react-https-redirect";

export function Global({ children }) {
  return (
    <HttpsRedirect>

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
    </HttpsRedirect>
  );
}

export function GlobalLive({ children }) {
  return (
    <Global>
      {children}

      <style jsx global>{`
        body {
          background: #000;
          color: #fff;
        }
      `}</style>
    </Global>
  );
}
