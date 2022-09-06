<script setup lang="ts">
import { computed } from "@vue/reactivity";
import { KeyLabel } from "../boardLayout";
import LabelRow from "./LayoutKeyLabelRow.vue";

interface KeyProps {
  label: KeyLabel;
  home?: boolean;
}

const props = defineProps<KeyProps>();

const rows = computed(() => {
  if (Array.isArray(props.label)) {
    return props.label;
  } else if (props.label) {
    return [props.label];
  } else {
    return [];
  }
});

const keySize = 55;
const keyStyle = computed(() => {
  const size = `${keySize}px`;
  return {
    width: size,
    height: size,
  };
});
</script>

<template>
  <div :style="keyStyle" :class="{ home: home }" class="key">
    <div v-for="(r, i) in rows" :key="i">
      <LabelRow :label="r" :row-count="rows.length" />
      <hr v-if="i < rows.length - 1" class="border-t-gray-500 w-full" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
$border-col: rgb(83, 83, 83);

.key {
  background-color: rgb(240, 240, 240);
  color: rgb(12, 12, 12);
  margin: 2px;
  border-radius: 7px;
  display: grid;
  justify-content: center;
  align-items: center;
  border: 3px solid $border-col;
}

.home {
  $accent-col: rgb(159, 190, 218);
  background-color: $accent-col;
  border-color: rgb(72, 66, 62);
}
</style>
