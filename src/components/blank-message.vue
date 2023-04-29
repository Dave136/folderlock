<template>
  <div
    class="flex flex-col justify-center items-center flex-1 gap-4"
    :class="class"
    ref="elementRef"
    @mouseenter="$emit('mouseenter')"
    @mouseleave="$emit('mouseleave')"
  >
    <slot name="icon" />
    <p class="text-sm">
      {{ message }}
    </p>
  </div>
</template>

<script lang="ts" setup>
interface Props {
  message: string;
  class?: string | string[];
}

interface Emits {
  (e: 'mouseenter', payload: MouseEvent): void;
  (e: 'mouseleave', payload: MouseEvent): void;
}

defineProps<Props>();
defineEmits<Emits>();
</script>
