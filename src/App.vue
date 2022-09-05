<script setup lang="ts">
import {
  alphaLayer,
  BoardLayer,
  navLayer,
  numLayer,
  symLayer,
  funLayer,
  winLayer,
  uniLayer,
} from "./boardLayout";
import LayoutLayer from "./Components/LayoutLayer.vue";
import { onMounted, onUnmounted, Ref, ref } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

let layer: Ref<BoardLayer | null> = ref(alphaLayer);
let layerChangeCount = ref(0);

const layers = [
  alphaLayer,
  null,
  navLayer,
  numLayer,
  symLayer,
  funLayer,
  uniLayer,
  winLayer,
];

if ((window as any).__TAURI__) {
  const unlisten: UnlistenFn[] = [];

  onMounted(async () => {
    unlisten.push(
      await listen<number>("layer", (ev) => {
        layer.value = layers[ev.payload];
        layerChangeCount.value++;
      })
    );
  });

  onUnmounted(() => {
    unlisten.forEach((fn) => fn());
  });
}
</script>

<template>
  <LayoutLayer
    v-if="layer"
    :layer="layer"
    :key="layerChangeCount"
    class="layout"
  />
</template>

<style lang="scss" scoped>
.layout {
  height: 100%;
  flex-grow: 1;
  align-self: stretch;
  background-color: rgba(73, 73, 73, 0.25);
  border: 5px solid rgba(43, 43, 43, 0.3);
  padding: 10px 5px 0 5px;
  border-radius: 3px;
  user-select: none;
}
</style>
