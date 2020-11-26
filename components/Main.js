import { useState, useEffect } from "react";

export function Main({ children }) {
  return (
    <>
      <main>{children}</main>

      <style jsx>{`
        main {
          flex: 1;
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
        }
      `}</style>
    </>
  );
}
