import React from "react";

export const highlightAllResources = (
  <>
    <link
      rel="stylesheet"
      href="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/styles/default.min.css"
    />
    <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/highlight.min.js"></script>
    <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/languages/javascript.min.js"></script>
    <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/languages/cpp.min.js"></script>
    <script src="https://unpkg.com/@highlightjs/cdn-assets@10.7.2/languages/glsl.min.js"></script>
  </>
);

export function HighlightAll() {
  return <script>hljs.highlightAll();</script>;
}
