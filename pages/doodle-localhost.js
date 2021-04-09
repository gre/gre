import React from "react";
import { Global } from "../components/Global";

export default function Home() {
  return (
    <Global>
      <div
        style={{
          padding: 10,
          width: "90vw",
          height: "90vh",
          boxSizing: "border-box",
        }}
      >
        <iframe
          title="hic et nunc SVG renderer"
          src="http://localhost:1234?creator=tz1cgQAQfECg5bPASYTMyJ9QJQjSUi8rfL67&viewer=false"
          sandbox="allow-scripts"
          scrolling="no"
          style={{ width: "100%", height: "100%" }}
        ></iframe>
      </div>
    </Global>
  );
}
