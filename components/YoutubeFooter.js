import { useState, useEffect } from "react";
import { useNextYoutube } from "../youtube";

export function YoutubeFooter() {
  const nextYoutube = useNextYoutube();
  if (!nextYoutube) return null;
  return (
    <p>
      <strong>🎙 Youtube Live on Saturday: </strong>“
      <a
        className="youtube"
        href={`https://www.youtube.com/watch?v=${nextYoutube.id}`}
        target="_blank"
      >
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

        a.youtube {
          text-decoration: underline;
        }
      `}</style>
    </p>
  );
}
