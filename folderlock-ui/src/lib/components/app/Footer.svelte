<script lang="ts" setup>
  import { InstagramLogo, GithubLogo, TwitterLogo } from 'phosphor-svelte';
  import VueSvg from '~/assets/vue.svg';
</script>

<footer class="flex mx-4 items-center justify-between col-span-full">
  <p class="text-zinc-500 text-[10px]">
    Built with
    <img class="w-2 inline" src={VueSvg} alt="Vue logo" /> and
    <img class="w-2 inline" src="/tauri.svg" alt="Tauri Logo" />
  </p>
  <p class="text-zinc-500 text-[10px]">By David Arenas - &copy; 2023</p>
  <div class="flex gap-2">
    <a href="#" target="_blank">
      <InstagramLogo size={12} class="text-zinc-500 hover:text-zinc-400" />
    </a>
    <a href="#" target="_blank">
      <GithubLogo size={12} class="text-zinc-500 hover:text-zinc-400" />
    </a>
    <a href="#" target="_blank">
      <TwitterLogo size={12} class="text-zinc-500 hover:text-zinc-400" />
    </a>
  </div>
</footer>
