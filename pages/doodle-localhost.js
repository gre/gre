import React from "react";

export default function Home() {
  return (
    <div style={{ padding: 20, height: "100vh", boxSizing: "border-box" }}>
      <iframe
        title="hic et nunc SVG renderer"
        src="http://localhost:1234?creator=tz1cgQAQfECg5bPASYTMyJ9QJQjSUi8rfL67&viewer=false"
        sandbox="allow-scripts"
        scrolling="no"
        style={{ width: "100%", height: "100%" }}
      ></iframe>
    </div>
  );
}
