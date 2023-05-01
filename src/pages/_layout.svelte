<script lang="ts">
  import Aside from '@/lib/components/app/Aside.svelte';
  import Footer from '@/lib/components/app/Footer.svelte';
  import globalStore from '@/lib/store/global';
  import { goto } from '@roxi/routify';
  import { onMount } from 'svelte';

  let showAside = false;
  let rootPasswordExists = false;

  let test = true;

  $: classes = rootPasswordExists ? 'grid-cols-[3rem_1fr]' : 'grid-cols-1';

  console.log($globalStore.osType);

  onMount(async () => {
    await globalStore.init();

    if (test) {
      showAside = true;
      rootPasswordExists = true;
      $goto('/');
      return;
    }

    showAside = false;
    rootPasswordExists = false;
  });
</script>

<div
  class="h-screen bg-[#282c34] text-zinc-100 grid grid-rows-[1fr_1.5rem] {classes}"
>
  {#if rootPasswordExists}
    <Aside />
  {/if}
  <main class="bg-[#1d1f23] text-zinc-400">
    <slot />
  </main>
  <Footer />
</div>
