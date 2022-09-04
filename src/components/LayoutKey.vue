<script setup lang="ts">
import { computed } from "@vue/reactivity";
import KeyLabelRow from "./KeyLabelRow.vue";
import { KeyLabel } from "../boardLayout";

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
      <KeyLabelRow :label="r" :row-count="rows.length" />
      <hr v-if="i < rows.length - 1" class="border-t-gray-500" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
$border-col: rgb(83, 83, 83);

.key {
  background-color: rgb(240, 240, 240);
  color: rgb(12, 12, 12);
  margin: 2px 3px;
  border-radius: 7px;
  display: grid;
  justify-content: center;
  align-items: center;
  border: 3px solid $border-col;
}

.home {
  border-color: rgb(72, 66, 62);
  background-color: rgb(246, 232, 201);
}
</style>
