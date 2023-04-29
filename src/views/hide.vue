<template>
  <section class="p-4 block h-full">
    <div class="flex flex-col justify-center items-center h-full gap-4">
      <ph-file :size="100" weight="fill" />
      <p class="text-sm">
        Drag and drop files or folders to the blank area to hide
      </p>
    </div>
  </section>
</template>

<script lang="ts" setup>
import { PhFile, PhFileArchive, PhFolder } from '@phosphor-icons/vue';
</script>
