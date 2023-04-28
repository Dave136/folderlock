<template>
  <section>
    <h2>My home view</h2>
  </section>
</template>

<script lang="ts" setup>
import { inject } from 'vue';
import { injectKeys } from '@/config/constants';

const hasApplicationPassword = inject(injectKeys.hasApplicationPassword);
</script>
