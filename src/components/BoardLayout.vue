<script setup lang="ts">
import LayoutLayer from "./LayoutLayer.vue";
import {
  alphaLayer,
  BoardLayer,
  navLayer,
  numLayer,
  symLayer,
} from "../boardLayout";
import {
  register,
  isRegistered,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { onMounted, onUnmounted, Ref, ref } from "vue";

let layer: Ref<BoardLayer | null> = ref(numLayer);
let layerChangeCount = ref(0);

const layerMap: Record<number, BoardLayer> = {
  13: alphaLayer,
  15: navLayer,
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

    //   for (let i = 13; i < 20; i++) {
    //     const shortcut = `F${i}`;
    //     if (!(await isRegistered(shortcut))) {
    //       register(shortcut, (s) => {
    //         console.warn(`Triggered shortcut '${s}'`);
    //       });
    //     }
    //   }
  });

  onUnmounted(async () => {
    await unregisterAll();
  });
}

// let x = 0;
// onMounted(() => {
//   setInterval(() => {
//     layer.value = x % 2 === 0 ? navLayer : alphaLayer;
//     x++;
//     layerChangeCount.value++;
//   }, 1500);
// });
</script>

<template>
  <LayoutLayer v-if="layer" :layer="layer" :key="layerChangeCount" />
</template>
