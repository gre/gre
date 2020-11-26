import { useState, useEffect } from "react";

export function Header({ children }) {
  return (
    <>
      <header>{children}</header>
      <style jsx>{`
        header {
          padding: 1rem 0;
          display: flex;
          flex-direction: column;
          align-items: center;
        }
      `}</style>
    </>
  );
}
