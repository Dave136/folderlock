<template>
  <div class="flex flex-col justify-center items-center flex-1 gap-4">
    <slot name="icon" />
    <p class="text-sm">
      {{ message }}
    </p>
  </div>
</template>

<script lang="ts" setup>
interface Props {
  message: string;
}

defineProps<Props>();
</script>
