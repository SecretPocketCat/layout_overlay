<script setup lang="ts">
import BoardLayout from "./components/BoardLayout.vue";
</script>

<template>
  <BoardLayout class="layout" />
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
