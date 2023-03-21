export const width = 297;
export const height = 210;
export const pad = 10;

export const params = [
  {
    id: "color_offset",
    name: "Color offset",
    type: "number",
    default: 0,
    options: {
      min: 0,
      max: 2,
      step: 1,
    },
  },
  {
    id: "color_cutoff",
    name: "Max colors",
    type: "number",
    options: {
      min: 1,
      max: 3,
      step: 1,
    },
  },
  {
    id: "layers_count",
    name: "Layers count",
    type: "number",
    default: 3,
    options: {
      min: 1,
      max: 5,
      step: 1,
    },
  },
  {
    id: "lightness",
    name: "Lightness",
    type: "number",
    default: 0,
    options: {
      min: -2,
      max: 3,
      step: 0.01,
    },
  },
  {
    id: "noise_effect",
    name: "distortion",
    type: "number",
    options: {
      min: 0,
      max: 1,
      step: 0.01,
    },
  },
  {
    id: "kaleidoscope",
    name: "Kaleidoscope",
    type: "boolean",
    default: false,
  },
  {
    id: "kaleidoscope_mod",
    name: "Kaleido-n",
    type: "number",
    default: 3,
    options: {
      min: 3,
      max: 16,
      step: 1
    }
  }
];
