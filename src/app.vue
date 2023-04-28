<template>
  <div
    class="h-screen bg-zinc-800 text-zinc-100 grid grid-cols-[5rem_1fr] grid-rows-[1fr_3rem]"
  >
    <aside v-if="showAside">
      <nav class="flex flex-col justify-center items-center gap-5 mt-2">
        <button>
          <ph-eye class="text-zinc-500" :size="42" weight="light" />
        </button>
        <button>
          <ph-lock class="text-zinc-500" :size="42" weight="light" />
        </button>
      </nav>
    </aside>
    <main class="bg-zinc-300 text-zinc-800">
      <router-view />
    </main>
    <app-footer />
  </div>
</template>

<script setup lang="ts">
import { onMounted, provide, ref } from 'vue';
import { PhEye, PhLock } from '@phosphor-icons/vue';
import { useRouter } from 'vue-router';
import { injectKeys } from '@/config/constants';
import { useStore } from './composables/use-tauri-store';
import AppFooter from './components/app-footer.vue';

const router = useRouter();

const hasApplicationPassword = ref(false);
const showAside = ref(false);
const store = useStore('password');

provide(injectKeys.hasApplicationPassword, hasApplicationPassword);

onMounted(async () => {
  const val = (await store.get()) as string;

  if (!val.length) {
    return router.push('/auth');
  }

  showAside.value = true;
});
</script>
