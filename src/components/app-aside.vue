<template>
  <aside>
    <nav
      class="flex flex-col justify-between items-center gap-4 mt-2 h-full mb-4"
    >
      <div class="flex flex-col justify-center gap-3">
        <button @click="$router.push('/')">
          <ph-eye
            :class="
              $route.path === '/'
                ? 'text-zinc-300'
                : 'text-zinc-500 hover:text-zinc-300'
            "
            :size="28"
            weight="light"
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
            weight="light"
          />
        </button>
      </div>
      <button class="mb-4" @click="$router.push('/settings')">
        <ph-gear
          :class="
            $route.path === '/settings'
              ? 'text-zinc-300'
              : 'text-zinc-500 hover:text-zinc-300'
          "
          :size="28"
          weight="light"
        />
      </button>
    </nav>
  </aside>
</template>

<script lang="ts" setup>
import { PhEye, PhLockSimple, PhGear } from '@phosphor-icons/vue';
</script>
