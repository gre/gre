import React from "react";
import PropTypes from "prop-types";
import Blur1D from "./Blur1D";
import directionForPassDefault from "./directionForPassDefault";

const Blur = ({
  width,
  height,
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
      <Blur1D
        width={width}
        height={height}
        pixelRatio={pixelRatio}
        direction={directionForPass(pass, factor, passes)}
      >
        {rec(pass - 1)}
      </Blur1D>
    );
  return rec(passes);
};

Blur.defaultProps = {
  passes: 2,
  directionForPass: directionForPassDefault
};

Blur.propTypes = {
  factor: PropTypes.number.isRequired,
  children: PropTypes.any.isRequired,
  passes: PropTypes.number,
  directionForPass: PropTypes.func,
  width: PropTypes.any,
  height: PropTypes.any,
  pixelRatio: PropTypes.number
};

export default Blur;
