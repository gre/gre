import React, { useState, useEffect } from "react";
import ControlSlider from "./ControlSlider";

const Sidebar = function ({
  mods,
  blockNumber,
  blocks,
  attributesRef,
  handleBlockChange,
}) {
  const [isVisible, toggleVisibility] = useState(true);
  const handleToggleVisibility = () => {
    toggleVisibility(!isVisible);
  };

  const [attributes, setAttributes] = useState({});
  useEffect(() => {
    const i = setInterval(() => {
      if (attributesRef.current) setAttributes(attributesRef.current());
    }, 100);
    return () => clearInterval(i);
  }, []);

  return (
    <>
      <style jsx global>
        {`
          .sidebar {
            width: 200px;
            border-left: #e0e0e0 1px solid;
            background-color: #fff;
            margin-right: 0px;
            transition: margin-right 0.3s ease-in-out;
            opacity: 0.4;
          }

          .sidebar.hidden {
            margin-right: -225px;
          }

          .sidebar .toggle-button {
            position: fixed;
            top: 10px;
            right: 10px;
            display: inline-block;
            width: 20px;
            height: 20px;
            cursor: pointer;
            z-index: 1;
          }

          .sidebar .toggle-button svg {
            fill: #ccc;
          }

          .sidebar .toggle-button:hover svg {
            fill: #fff;
          }

          .sidebar.hidden .toggle-button {
            transform: rotate(180deg);
          }

          .sidebar.hidden .toggle-button svg {
            fill: #000;
          }

          .sidebar .section-header {
            height: 40px;
            background: #000;
            color: #fff;
            line-height: 40px;
            text-align: center;
            font-size: 16px;
          }

          .sidebar .section-body {
            padding: 20px;
          }

          .sidebar .custom-attribute {
            margin-bottom: 15px;
          }

          .sidebar .content-header {
            display: block;
            font-weight: bold;
            margin: 5px 0;
          }
        `}
      </style>

      <div className={`sidebar ${isVisible ? "" : "hidden"}`}>
        <div className="toggle-button" onClick={handleToggleVisibility}>
          <svg
            width="20"
            height="20"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 484.4 479.2"
          >
            <path d="M382.4 479.2h102V0h-102v479.2zM338 239.6L206.1 126.3v64.9H0v97.9h206.1V353" />
          </svg>
        </div>

        <div className="section-header">Change Block</div>
        <div className="section-body">
          <ControlSlider
            modValue={blockNumber}
            modValueMin="0"
            modValueMax={blocks.length - 1}
            modValueStep="1"
            onChange={(e) => {
              handleBlockChange(e);
            }}
          />
          {parseInt(blocks[blockNumber].number)}
        </div>

        <div className="section-header">Change Style</div>
        <div className="section-body">
          {mods.map(({ key, value, set }) => {
            return (
              <ControlSlider
                key={key}
                controlLabel={key}
                modValue={value}
                onChange={set}
              />
            );
          })}
        </div>

        <div className="section-header">Custom Attributes</div>
        <div className="section-body">
          {attributes.attributes
            ? attributes.attributes.map((attribute, index) => {
                return (
                  <div className="custom-attribute" key={index}>
                    <div className="content-header">{attribute.trait_type}</div>
                    <div>{attribute.value}</div>
                  </div>
                );
              })
            : ""}
        </div>
      </div>
    </>
  );
};
export default Sidebar;
