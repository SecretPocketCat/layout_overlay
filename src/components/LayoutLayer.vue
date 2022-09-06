<script setup lang="ts">
import { BoardLayer, KeyLabel } from "../boardLayout";
import LayoutKey from "./LayoutKey.vue";

interface LayerProps {
  layer: BoardLayer;
}

interface LayerColumn {
  keys: KeyLabel[];
  styleClass?: string;
}

interface LayerThumb {
  key: KeyLabel;
  homeKey?: boolean;
  styleClass?: string;
}

const props = defineProps<LayerProps>();

const columns: LayerColumn[] = [
  // left
  {
    keys: props.layer.left.pinkyOuter as any,
    styleClass: "mt-[100px]",
  },
  {
    keys: props.layer.left.pinky as any,
    styleClass: "mt-[55px]",
  },
  {
    keys: props.layer.left.ring as any,
    styleClass: "mt-7",
  },
  {
    keys: props.layer.left.middle as any,
  },
  {
    keys: props.layer.left.index as any,
    styleClass: "mt-6",
  },
  {
    keys: props.layer.left.indexOuter as any,
    styleClass: "mt-8 mr-4",
  },
  // right
  {
    keys: props.layer.right.indexOuter as any,
    styleClass: "mt-8 ml-4",
  },
  {
    keys: props.layer.right.index as any,
    styleClass: "mt-6",
  },
  {
    keys: props.layer.right.middle as any,
  },
  {
    keys: props.layer.right.ring as any,
    styleClass: "mt-7 rotate-10",
  },
  {
    keys: props.layer.right.pinky as any,
    styleClass: "mt-[55px]",
  },
  {
    keys: props.layer.right.pinkyOuter as any,
    styleClass: "mt-[100px]",
  },
];

function isHomeRow(col: number, row: number): boolean {
  return ((col >= 1 && col <= 4) || (col >= 7 && col <= 10)) && row == 1;
}

const thumbShift = "-translate-y-6";
const thumbs: LayerThumb[] = [
  // left
  {
    key: props.layer.left.thumb[0],
    styleClass: "transform ml-[185px]",
  },
  {
    key: props.layer.left.thumb[1],
    homeKey: true,
    styleClass: "mt-[8px] ml-[6px] rotate-[12deg]",
  },
  {
    key: props.layer.left.thumb[2],
    styleClass: "mt-[27px] ml-[4px] rotate-[24deg]",
  },
  // right
  {
    key: props.layer.right.thumb[0],
    styleClass: "ml-[45px] mt-[27px] mr-[4px] rotate-[-24deg]",
  },
  {
    key: props.layer.right.thumb[1],
    homeKey: true,
    styleClass: "mt-[8px] mr-[6px] rotate-[-12deg]",
  },
  {
    key: props.layer.right.thumb[2],
  },
];
</script>

<template>
  <div class="layer">
    <div class="flex flex-row">
      <div
        v-for="(c, i) in columns"
        :class="c.styleClass"
        class="flex flex-col"
      >
        <LayoutKey
          v-for="(k, j) in c.keys"
          :key="j"
          :label="k"
          :home="isHomeRow(i, j)"
        />
      </div>
    </div>

    <div class="flex flex-row thumbs">
      <LayoutKey
        v-for="(t, i) in thumbs"
        :key="i"
        :label="t.key"
        :home="t.homeKey"
        :class="`flex flex-col ${t.styleClass} ${thumbShift}`"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
.thumbs {
  height: 75px;
}
</style>
