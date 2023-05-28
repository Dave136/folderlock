<script lang="ts">
  import { url, isActive, goto } from '@roxi/routify';
  import { Eye, LockSimple, Gear } from 'phosphor-svelte';

  $: isPathActive = (path: string) =>
    $isActive(path) ? 'text-zinc-300' : 'text-zinc-500 hover:text-zinc-300';
</script>

<template>
  <aside>
    <nav
      class="flex flex-col justify-between items-center gap-4 mt-2 h-full mb-4"
    >
      <div class="flex flex-col justify-center gap-3">
        <button on:click={() => $goto('./')}>
          <Eye class={isPathActive('./index')} size={28} weight="light" />
        </button>
        <button on:click={() => $url('/encrypt')}>
          <LockSimple
            class={isPathActive('/encrypt')}
            size={28}
            weight="light"
          />
        </button>
      </div>
      <button class="mb-4" on:click={() => $goto('./settings')}>
        <Gear class={isPathActive('./settings')} size={28} weight="light" />
      </button>
    </nav>
  </aside>
</template>
