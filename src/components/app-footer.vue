<template>
  <footer class="flex mx-6 items-center justify-between col-span-full">
    <p class="text-zinc-500 text-sm">
      Built with
      <img class="w-3 inline" src="@/assets/vue.svg" alt="Vue logo" /> and
      <img class="w-3 inline" src="/tauri.svg" alt="Tauri Logo" />
    </p>
    <p class="text-zinc-500 text-sm">By David Arenas - &copy; 2023</p>
    <div class="flex gap-4">
      <a href="#" target="_blank">
        <ph-instagram-logo />
      </a>
      <a href="#" target="_blank">
        <ph-github-logo />
      </a>
      <a href="#" target="_blank">
        <ph-twitter-logo />
      </a>
    </div>
  </footer>
</template>

<script lang="ts" setup>
import {
  PhInstagramLogo,
  PhGithubLogo,
  PhTwitterLogo,
} from '@phosphor-icons/vue';
</script>
