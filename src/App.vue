<script setup lang="ts">
import { layout, BoardLayer, getLayerFromIndex } from "./boardLayout";
import { appLayouts, LayoutApp } from "./appLayouts";
import LayoutLayer from "./Components/LayoutLayer.vue";
import { onMounted, onUnmounted, Ref, ref } from "vue";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { computed } from "@vue/reactivity";

let layerIndex: Ref<number> = ref(0);
let appIndex: Ref<LayoutApp | null> = ref(null);
let layerChangeCount = ref(0);

const layer = computed(() => {
  const layer = getLayerFromIndex(layerIndex.value);
  const appLayout =
    appIndex.value != null ? appLayouts[appIndex.value][layer] : null;
  return appLayout || layout[layer];
});

if ((window as any).__TAURI__) {
  const unlisten: UnlistenFn[] = [];

  onMounted(async () => {
    unlisten.push(
      await listen<number>("layer", (ev) => {
        layerIndex.value = ev.payload;
        layerChangeCount.value++;
      }),
      await listen<number>("app", (ev) => {
        if (ev.payload != appIndex.value) {
          console.warn("app change", ev.payload);
          appIndex.value = ev.payload;
          layerChangeCount.value++;
        }
      })
    );
  });

  onUnmounted(() => {
    unlisten.forEach((fn) => fn());
  });
}
</script>

<template>
  <div>
    <LayoutLayer
      v-if="layer"
      :layer="layer"
      :key="layerChangeCount"
      class="layout"
    />
  </div>
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
