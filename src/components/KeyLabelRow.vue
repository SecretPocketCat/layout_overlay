<script setup lang="ts">
import { computed } from "@vue/reactivity";
import { isStyledLabel, LabelValue } from "../boardLayout";

interface RowProps {
  label: LabelValue;
  rowCount: number;
}

const props = defineProps<RowProps>();

const labelText = computed(() => {
  let txt = null;

  if (isStyledLabel(props.label)) {
    txt = props.label.label;
  } else {
    txt = props.label;
  }

  return txt?.toUpperCase();
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
    filter: "grayscale(1)",
  };

  return isStyledLabel(props.label)
    ? Object.assign({}, st, props.label.style)
    : st;
});
</script>

<template>
  <Transition mode="out-in" appear>
    <div :key="labelText" :style="labelStyle" class="row">
      {{ labelText }}
    </div>
  </Transition>
</template>

<style lang="scss" scoped>
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
