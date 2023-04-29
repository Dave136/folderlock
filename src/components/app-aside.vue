<template>
  <aside>
    <nav class="flex flex-col justify-center items-center gap-4 mt-2">
      <button @click="$router.push('/')">
        <ph-eye
          :class="
            $route.path === '/'
              ? 'text-zinc-300'
              : 'text-zinc-500 hover:text-zinc-300'
          "
          :size="28"
          weight="regular"
        />
      </button>
      <button @click="$router.push('/encrypt')">
        <ph-lock-simple
          :class="
            $route.path === '/encrypt'
              ? 'text-zinc-300'
              : 'text-zinc-500 hover:text-zinc-300'
          "
          :size="28"
          weight="regular"
        />
      </button>
    </nav>
  </aside>
</template>

<script lang="ts" setup>
import { PhEye, PhLockSimple } from '@phosphor-icons/vue';
</script>
