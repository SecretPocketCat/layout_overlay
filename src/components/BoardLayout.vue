<script setup lang="ts">
import LayoutLayer from "./LayoutLayer.vue";
import {
  alphaLayer,
  BoardLayer,
  navLayer,
  numLayer,
  symLayer,
  funLayer,
  winLayer,
  uniLayer,
} from "../boardLayout";
import {
  register,
  isRegistered,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { onMounted, onUnmounted, Ref, ref } from "vue";

let layer: Ref<BoardLayer | null> = ref(alphaLayer);
let layerChangeCount = ref(0);

const layerMap: Record<number, BoardLayer> = {
  13: alphaLayer,
  // 14: qwertyLayer,
  15: navLayer,
  16: numLayer,
  17: symLayer,
  18: funLayer,
  19: uniLayer,
  20: winLayer,
};

if ((window as any).__TAURI__) {
  onMounted(async () => {
    Object.entries(layerMap).forEach(async (x) => {
      const shortcut = `F${x[0]}`;
      if (!(await isRegistered(shortcut))) {
        register(shortcut, (s) => {
          layer.value = x[1];
          console.warn(
            `Triggered shortcut '${s}', ${layer.value.left.index[1]}`
          );
          layerChangeCount.value++;
        });
      }
    });
  });

  onUnmounted(async () => {
    await unregisterAll();
  });
}
</script>

<template>
  <LayoutLayer v-if="layer" :layer="layer" :key="layerChangeCount" />
</template>
