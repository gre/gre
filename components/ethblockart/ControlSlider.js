import React from "react";

const ControlSlider = function (props) {
  const handleModChange = (event) => {
    props.onChange(parseFloat(event.target.value));
  };

  return (
    <>
      <style jsx global>{`
        .control-slider {
          margin: 5px 0;
          font-size: 12px;
        }

        .control-slider label {
          display: block;
        }
        .control-slider .control-input {
          display: flex;
        }

        .control-slider .value-label {
          font-size: 12px;
          width: 50px;
          height: 16px;
          line-height: 16px;
          margin: 8px 0;
        }

        .control-slider input[type="range"] {
          flex-grow: 1;
          height: 21px;
          -webkit-appearance: none;
          margin: 6px 0;
          width: 100%;
        }
        .control-slider input[type="range"]:focus {
          outline: none;
        }
        .control-slider input[type="range"]::-webkit-slider-runnable-track {
          width: 100%;
          height: 5px;
          cursor: pointer;
          animate: 0.2s;
          box-shadow: 0px 0px 0px #000000;
          background: #6c6c6c;
          border-radius: 5px;
          border: 0px solid #000000;
        }
        .control-slider input[type="range"]::-webkit-slider-thumb {
          box-shadow: 0px 0px 0px #000000;
          border: 0px solid #000000;
          height: 15px;
          width: 15px;
          border-radius: 8px;
          background: #000000;
          cursor: pointer;
          -webkit-appearance: none;
          margin-top: -5px;
        }
        .control-slider
          input[type="range"]:focus::-webkit-slider-runnable-track {
          background: #6c6c6c;
        }
        .control-slider input[type="range"]::-moz-range-track {
          width: 100%;
          height: 5px;
          cursor: pointer;
          animate: 0.2s;
          box-shadow: 0px 0px 0px #000000;
          background: #6c6c6c;
          border-radius: 5px;
          border: 0px solid #000000;
        }
        .control-slider input[type="range"]::-moz-range-thumb {
          box-shadow: 0px 0px 0px #000000;
          border: 0px solid #000000;
          height: 15px;
          width: 15px;
          border-radius: 8px;
          background: #000000;
          cursor: pointer;
        }
        .control-slider input[type="range"]::-ms-track {
          width: 100%;
          height: 5px;
          cursor: pointer;
          animate: 0.2s;
          background: transparent;
          border-color: transparent;
          color: transparent;
        }
        .control-slider input[type="range"]::-ms-fill-lower {
          background: #6c6c6c;
          border: 0px solid #000000;
          border-radius: 10px;
          box-shadow: 0px 0px 0px #000000;
        }
        .control-slider input[type="range"]::-ms-fill-upper {
          background: #6c6c6c;
          border: 0px solid #000000;
          border-radius: 10px;
          box-shadow: 0px 0px 0px #000000;
        }
        .control-slider input[type="range"]::-ms-thumb {
          margin-top: 1px;
          box-shadow: 0px 0px 0px #000000;
          border: 0px solid #000000;
          height: 15px;
          width: 15px;
          border-radius: 8px;
          background: #000000;
          cursor: pointer;
        }
        .control-slider input[type="range"]:focus::-ms-fill-lower {
          background: #6c6c6c;
        }
        .control-slider input[type="range"]:focus::-ms-fill-upper {
          background: #6c6c6c;
        }
      `}</style>

      <div className="control-slider">
        <label>{props.controlLabel}</label>
        <div className="control-input">
          <div className="value-label">{props.modValue}</div>
          <input
            id="controlSlider"
            type="range"
            min={props.modValueMin || 0}
            max={props.modValueMax || 1}
            defaultValue={props.modValue || 0.5}
            step={props.modValueStep || 0.001}
            onChange={handleModChange}
          />
        </div>
      </div>
    </>
  );
};
export default ControlSlider;
