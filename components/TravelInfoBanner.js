import React from "react";

export function TravelInfoBanner() {
  return (
    <div className="banner">
      <style jsx>{`
          .banner {
            font-size: 1.4em;
            letter-spacing: 0.1em;
            background: #c00;
            color: #fff;
          }
          .banner a {
            display: flex;
            flex-direction: row;
            max-width: 700px;
            margin: 0 auto;
            padding: 0.5em;
          }
          .banner a:hover {
            text-decoration: none;
          }
          .banner img {
            border: 4px solid white;
            margin-right: 0.5em;
            max-width: 200px;
            object-fit: cover;
          }
      `}</style>
      <a target="_blank" href="https://www.youtube.com/playlist?list=PLe93qXGkp4ViwKJuBxXGau8IUikQf3JyI">
        <img src="/images/thumbnail-vlog-china-shenao.jpg" />
        <div>
          I am currently on a linguistic &amp; cultural trip in China for a gap year with family.
          Follow our adventure: <span style={{ textDecorationLine: "underline" }}>youtube.com/@greweb</span>
        </div>
      </a>
    </div>
  );
}
