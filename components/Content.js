import React from "react";

export function Content({ children }) {
  return (
    <div className="content">
      {children}
      <style jsx>{`
        .content {
          max-width: 600px;
          padding: 0 10px;
        }
      `}</style>
    </div>
  );
}
