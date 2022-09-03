<script setup lang="ts">
import LayoutLayer from "./components/LayoutLayer.vue";
import { alphaLayer } from "./boardLayout";
</script>

<template>
  <LayoutLayer :layer="alphaLayer" />
</template>

<style scoped></style>
