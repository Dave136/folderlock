<template>
  <section class="p-4 flex flex-col h-full" :class="class">
    <slot />
  </section>
</template>

<script lang="ts" setup>
defineProps<{ class?: string }>();
</script>
