// @flow
import React from "react";
import me from "../me";

const MeBlock = () => {
  return (
    <>
      <style jsx>{`
        .block {
          display: flex;
          flex-direction: row;
          align-items: center;
        }
        .block .right {
          padding: 10px;
        }
        .block .social {
          margin-top: 10px;
        }
        .block .social a {
          padding: 10px;
        }
        .block .social img {
          height: 20px;
        }
      `}</style>
      <div className="block">
        <img src={`${me.thumbnailDomain}${me.thumbnail}`} width="100" />
        <div className="right">
          <div className="description">{me.description}</div>
          <div className="social">
            {me.social.map(({ id, url, icon }) => (
              <a key={id} href={url}>
                <img alt="" src={icon} />
              </a>
            ))}
          </div>
        </div>
      </div>
    </>
  );
};

export default MeBlock;
