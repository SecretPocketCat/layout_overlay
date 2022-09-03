<script setup lang="ts">
import { computed } from "@vue/reactivity";
import KeyLabelRow from "./KeyLabelRow.vue";
import { KeySize } from "../constants";
import { KeyLabel } from "../boardLayout";

interface KeyProps {
  label: KeyLabel;
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

const keyStyle = computed(() => {
  const padding = 2;
  const size = `${KeySize - padding / 2}px`;
  return {
    width: size,
    height: size,
    padding: `${padding}px`,
  };
});
</script>

<template>
  <div class="key" :style="keyStyle">
    <KeyLabelRow
      v-for="(r, i) in rows"
      :key="i"
      :label="r"
      :row-count="rows.length"
    />
  </div>
</template>

<style lang="scss" scoped>
.key {
  background-color: rgb(225, 225, 225);
  color: rgb(12, 12, 12);
  margin: 2px 3px;
  border-radius: 7px;
  display: grid;
  justify-content: center;
  align-items: center;
}
</style>
