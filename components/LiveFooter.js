import React from "react";

export function LiveFooter({ Day }) {
  return (
    <div>
      <style jsx>{`
        a {
          color: inherit;
          text-decoration: underline;
        }
        p {
          padding-bottom: 20px;
        }
        .nft {
          position: absolute;
          color: #f60;
        }
      `}</style>
      <p>
        <strong>🎙 Livecoded at </strong>
        <a href={`https://twitch.tv/greweb`} target="_blank" rel="noreferrer">
          twitch.tv/greweb
        </a>
      </p>
      {Day.nfts ? (
        <p className="nft">
          <span style={{ marginRight: 10 }}>👋</span>
          <strong>Support this work with blockchain!</strong>
          <ul>
            {Day.nfts.map(({ url, text }) => {
              return (
                <li key={url}>
                  <a href={url}>{text}</a>
                </li>
              );
            })}
          </ul>
        </p>
      ) : null}
    </div>
  );
}
