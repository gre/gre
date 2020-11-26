import React from "react";
import PropTypes from "prop-types";
import BlurV1D from "./BlurV1D";
import directionForPassDefault from "./directionForPassDefault";

const BlurV = ({
  width,
  height,
  map,
  pixelRatio,
  factor,
  children,
  passes,
  directionForPass
}) => {
  const rec = pass =>
    pass <= 0 ? (
      children
    ) : (
      <BlurV1D
        width={width}
        height={height}
        map={map}
        pixelRatio={pixelRatio}
        direction={directionForPass(pass, factor, passes)}
      >
        {rec(pass - 1)}
      </BlurV1D>
    );
  return rec(passes);
};

BlurV.defaultProps = {
  passes: 2,
  directionForPass: directionForPassDefault
};

BlurV.propTypes = {
  factor: PropTypes.number.isRequired,
  children: PropTypes.any.isRequired,
  passes: PropTypes.number,
  directionForPass: PropTypes.func,
  map: PropTypes.any.isRequired,
  width: PropTypes.any,
  height: PropTypes.any,
  pixelRatio: PropTypes.number
};

export default BlurV;
