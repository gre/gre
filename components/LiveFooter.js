import { useState, useEffect } from "react";

export function LiveFooter() {
  return (
    <p>
      <strong>🎙 Livestream each Saturday: </strong>“
      <a className="live" href={`https://twitch.tv/greweb`} target="_blank">
        It's Shaderday!
      </a>
      ”
      <style jsx>{`
        a {
          color: inherit;
          text-decoration: none;
        }

        a:hover,
        a:active {
          text-decoration: underline;
        }

        a.live {
          text-decoration: underline;
        }
      `}</style>
    </p>
  );
}
