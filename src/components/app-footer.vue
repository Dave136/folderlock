<template>
  <footer class="flex mx-4 items-center justify-between col-span-full">
    <p class="text-zinc-500 text-[10px]">
      Built with
      <img class="w-2 inline" src="@/assets/vue.svg" alt="Vue logo" /> and
      <img class="w-2 inline" src="/tauri.svg" alt="Tauri Logo" />
    </p>
    <p class="text-zinc-500 text-[10px]">By David Arenas - &copy; 2023</p>
    <div class="flex gap-2">
      <a href="#" target="_blank">
        <ph-instagram-logo
          :size="12"
          class="text-zinc-500 hover:text-zinc-400"
        />
      </a>
      <a href="#" target="_blank">
        <ph-github-logo :size="12" class="text-zinc-500 hover:text-zinc-400" />
      </a>
      <a href="#" target="_blank">
        <ph-twitter-logo :size="12" class="text-zinc-500 hover:text-zinc-400" />
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
