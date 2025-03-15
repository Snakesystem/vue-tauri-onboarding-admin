<script setup>
import Header from "./components/Header.vue";
</script>

<template>
  <Header v-if="$route.meta.showHeader"></Header>
  <RouterView/>
</template>

<style scoped>

</style>