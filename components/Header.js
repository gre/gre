import React from "react";

export function Header({ children }) {
  return (
    <>
      <header>{children}</header>
      <style jsx>{`
        header {
          margin: 1rem 0;
          display: flex;
          flex-direction: column;
          align-items: center;
          font-size: 16px;
        }
      `}</style>
    </>
  );
}
