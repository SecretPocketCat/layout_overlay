import { Layer, OptionalLayout } from "./boardLayout";

export enum LayoutApp {
  // Code,
  Blender,
  DaVinciResolve,
}

enum DanceModifier {
  Select = "Sel",
  Selection = "Selection",
  Extend = "➕",
  Backward = "Back",
  Register = "💾",
  Cursor = "|",
  Search = "🔍",
  Up = "▲",
  Down = "⯆",
  Left = "⯇",
  Right = "⯈",
  Insert = "🞃_",
  Append = "_🞃",
  Word = "_",
  Line = "⋍",
}

function joinLabel(...parts: string[]): string {
  return parts.join(" ");
}

const codeLayout: OptionalLayout = {
  ALPHA: {
    left: {
      pinkyOuter: [joinLabel(DanceModifier.Search, DanceModifier.Selection)],
      pinky: [["Seek", DanceModifier.Backward], ["Goto", "Ext"], null],
      ring: [
        ["SeekInc", DanceModifier.Backward],
        [DanceModifier.Insert, "⋍🞃"],
        "Clear Sel",
      ],
      middle: [
        `Match ${DanceModifier.Cursor}`,
        [DanceModifier.Append, "🞃⋍"],
        joinLabel("Copy", DanceModifier.Register),
      ],
      index: ["todo", "🦘", joinLabel("Paste", DanceModifier.Register)],
      indexOuter: [null, ["Next match", "Prev match"]],
      thumb: [
        ["ESC", Layer.win],
        [DanceModifier.Search, DanceModifier.Extend],
        ["TAB", Layer.uni],
      ],
    },
    right: {
      indexOuter: [null, "//"],
      index: [
        joinLabel(DanceModifier.Select, "Object"),
        [joinLabel("Word", DanceModifier.Backward), DanceModifier.Extend],
        "Repeat Edit",
      ],
      middle: [
        joinLabel(DanceModifier.Select, "Inner Object"),
        [
          joinLabel(DanceModifier.Select, DanceModifier.Down),
          joinLabel(DanceModifier.Select, "Line", DanceModifier.Down),
        ],
        "↳",
      ],
      ring: [
        "todo",
        [
          joinLabel(DanceModifier.Select, DanceModifier.Up),
          joinLabel(DanceModifier.Select, "Line", DanceModifier.Up),
        ],
        "↱",
      ],
      pinky: ["todo", [DanceModifier.Word, DanceModifier.Extend], null],
      pinkyOuter: [
        [joinLabel(DanceModifier.Word, "End"), DanceModifier.Extend],
      ],
      thumb: [
        ["ENTER", Layer.fun],
        ["⇽", Layer.num],
        ["⇽WRD", Layer.sym],
      ],
    },
  },
  NAV: null,
  NUM: null,
};

const blenderLayout: OptionalLayout = {
  ALPHA: null,
  NAV: null,
  NUM: null,
};

const resolveLayout: OptionalLayout = {
  ALPHA: null,
  NAV: null,
  NUM: null,
};

export const appLayouts: { [key in LayoutApp]: OptionalLayout } = {
  // [LayoutApp.Code]: codeLayout,
  [LayoutApp.Blender]: blenderLayout,
  [LayoutApp.DaVinciResolve]: resolveLayout,
};
