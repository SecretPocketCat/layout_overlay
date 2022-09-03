<script setup lang="ts">
import { computed } from "@vue/reactivity";
import { isStyledLabel, LabelValue } from "../boardLayout";

interface RowProps {
  label: LabelValue;
  rowCount: number;
}

const props = defineProps<RowProps>();

const labelText = computed(() => {
  if (isStyledLabel(props.label)) {
    return props.label.label;
  } else {
    return props.label;
  }
});

const labelStyle = computed(() => {
  let size = props.rowCount > 1 || labelText.value.length > 2 ? 20 : 30;
  if (labelText.value.length >= 8) {
    size = 9;
  } else if (labelText.value.length >= 6) {
    size = 12;
  } else if (labelText.value.length >= 4) {
    size = 14;
  }

  const st = {
    fontSize: `${size}px`,
  };

  return isStyledLabel(props.label)
    ? Object.assign({}, st, props.label.style)
    : st;
});
</script>

<template>
  <div :style="labelStyle">
    {{ labelText }}
  </div>
</template>

<style lang="scss" scoped></style>
