<template>
  <td class="px-6 py-3 whitespace-nowrap text-sm text-gray-400" :class="class">
    <slot />
  </td>
  <!-- <td class="text-sm text-gray-400 font-light px-6 py-3 whitespace-nowrap">
    Content
  </td>
  <td class="text-sm text-gray-400 font-light px-6 py-3 whitespace-nowrap">
    Content
  </td>
  <td class="text-sm text-gray-400 font-light px-6 py-3 whitespace-nowrap">
    Content
  </td> -->
</template>

<script lang="ts" setup>
defineProps<{ class?: string }>();
</script>
