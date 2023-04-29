<template>
  <button
    :type="type"
    class="px-5 border border-zinc-500 py-1 rounded-md transition text-sm text-zinc-500 hover:border-zinc-400 hover:text-zinc-400"
    :class="classes"
    @click="$emit('click')"
  >
    <slot name="left" />
    <slot />
    <slot name="right" />
  </button>
</template>

<script lang="ts" setup>
import { computed, useSlots } from 'vue';

interface Props {
  type: 'button' | 'submit';
  class?: string;
}

interface Emits {
  (e: 'click'): void;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'button',
  class: '',
});

defineEmits<Emits>();

const slots = useSlots();

const classes = computed(() => ({
  'flex items-center gap-1': slots.left || slots.right,
  [props.class]: true,
}));
</script>
