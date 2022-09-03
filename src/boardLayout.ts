import { FixedArray } from "./array";

interface StyledLabel {
  label: string;
  style: object;
}

export type LabelValue = string | StyledLabel;

export function isStyledLabel(val: LabelValue): val is StyledLabel {
  return (val as any).style !== undefined;
}

export type KeyLabel = null | LabelValue | LabelValue[];

export interface BoardHalf {
  pinkyOuter: FixedArray<KeyLabel, 1>;
  pinky: FixedArray<KeyLabel, 3>;
  ring: FixedArray<KeyLabel, 3>;
  middle: FixedArray<KeyLabel, 3>;
  index: FixedArray<KeyLabel, 3>;
  indexOuter: FixedArray<KeyLabel, 2>;
  thumb: FixedArray<KeyLabel, 3>;
}

export interface BoardLayer {
  left: BoardHalf;
  right: BoardHalf;
}

export enum Layer {
  alpha = "ALPHA",
  nav = "NAV",
  num = "NUM",
  sym = "SYM",
  fun = "FUN",
  win = "WIN",
  uni = "UNI",
}

export const alphaLayer: BoardLayer = {
  left: {
    pinkyOuter: ["Z"],
    pinky: ["J", "A", "ALT"],
    ring: [
      "W",
      "R",
      [
        "CTRL",
        {
          label: ",",
          style: {
            fontSize: "2em",
            transform: "translateY(-10px)",
          },
        },
      ],
    ],
    middle: ["F", "S", ["SHFT", "("]],
    index: ["P", "T", "V"],
    indexOuter: ["G", "D"],
    thumb: [
      ["ESC", Layer.win],
      [
        {
          label: "˽",
          style: {
            fontSize: "3em",
            transform: "translateY(-15px)",
          },
        },
        Layer.nav,
      ],
      ["TAB", Layer.uni],
    ],
  },
  right: {
    indexOuter: ["C", "H"],
    index: ["L", "N", "M"],
    middle: ["U", "E", ["SHFT", ")"]],
    ring: [
      "Y",
      "I",
      [
        "CTRL",
        {
          label: ".",
          style: {
            fontSize: "2em",
            transform: "translateY(-10px)",
          },
        },
      ],
    ],
    pinky: ["B", "O", ["ALT", "?"]],
    pinkyOuter: ["K"],
    thumb: [
      ["ENTER", Layer.fun],
      ["⇽", Layer.num],
      ["⇽WRD", Layer.sym],
    ],
  },
};
