import { useState, useEffect, useRef } from "react";
import vec3 from "gl-vec3";
import mat3 from "gl-mat3";

// from https://github.com/gre/memocart/blob/master/src/Game/Logic/debugFreeControls.js

function threshold(value, limit) {
  if (Math.abs(value) < limit) return 0;
  return value;
}
function setMatRot(rot, rotX, rotY) {
  const cx = Math.cos(rotX);
  const sx = Math.sin(rotX);
  const cy = Math.cos(rotY);
  const sy = Math.sin(rotY);
  // prettier-ignore
  mat3.multiply(
    rot,
    [
      1, 0, 0,
      0, cx, sx,
      0, -sx, cx
    ],
    [
      cy, 0, sy,
      0, 1, 0,
      -sy, 0, cy
    ]
  );
  mat3.transpose(rot, rot);
}

const defaults = {
  moveSpeed: 0.1,
  rotSpeed: 0.03,
};

export function useXboxFreeControls(arg) {
  const { moveSpeed, rotSpeed } = { ...defaults, ...arg };

  const [state, setState] = useState(() => ({
    rotX: 0,
    rotY: 0,
    rotation: mat3.create(),
    origin: [0, 0, -5],
    buttonsPressed: [0, 0, 0, 0],
    buttonsPressCount: [0, 0, 0, 0],
  }));

  useEffect(() => {
    function loop(t) {
      requestAnimationFrame(loop);
      const gamepad = navigator.getGamepads ? navigator.getGamepads()[0] : null;
      if (!gamepad) return;
      setState(
        ({
          rotX,
          rotY,
          rotation,
          origin,
          buttonsPressCount,
          buttonsPressed,
        }) => {
          let move = [0, 0, 0];
          const { axes, buttons } = gamepad;
          if (axes.length >= 2) {
            move[0] += moveSpeed * threshold(axes[0], 0.2);
            move[2] -= moveSpeed * threshold(axes[1], 0.2);
          }
          if (axes.length >= 4) {
            rotY += rotSpeed * threshold(axes[2], 0.2);
            rotX += rotSpeed * threshold(axes[3], 0.2);
          }
          if (buttons.length > 7) {
            move[1] += moveSpeed * (buttons[7].value - buttons[6].value);
            buttonsPressCount = buttonsPressCount.map(
              (count, i) =>
                count + Number(buttons[i].pressed && !buttonsPressed[i])
            );
            buttonsPressed = buttons
              .slice(0, buttonsPressed.length)
              .map((btn) => btn.pressed);
          }

          const vector = vec3.create();
          vec3.transformMat3(vector, move, rotation);
          vec3.add(origin, origin, vector);
          setMatRot(rotation, rotX, rotY);
          return {
            rotX,
            rotY,
            rotation,
            origin,
            buttonsPressed,
            buttonsPressCount,
          };
        }
      );
    }
    requestAnimationFrame(loop);
  }, []);

  return state;
}
