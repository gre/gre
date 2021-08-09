import React from "react";
export function SourceCodeFooter({ Day }) {
  return (
    <>
      <footer>
        <a
          target="_blank"
          href={`https://github.com/gre/gre/blob/master/shaderdays/${(
            Day.n + ""
          ).padStart(3, "0")}.js`} rel="noreferrer"
        >
          source code
        </a>
      </footer>
      <style jsx>{`
        footer {
          padding: 1rem 0;
          font-size: 0.8rem;
          font-style: italic;
        }
      `}</style>
    </>
  );
}
