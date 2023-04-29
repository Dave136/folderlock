<template>
  <section class="p-4 block h-full">
    <div class="flex flex-col justify-center items-center h-full gap-4">
      <ph-drop :size="100" weight="fill" />
      <p class="text-sm">Encrypt files to mantain it secure</p>
    </div>
  </section>
</template>

<script lang="ts" setup>
import { PhDrop } from '@phosphor-icons/vue';
</script>
