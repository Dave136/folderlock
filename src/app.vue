<template>
  <div
    class="h-screen bg-[#282c34] text-zinc-100 grid grid-cols-[3rem_1fr] grid-rows-[1fr_1.5rem]"
  >
    <app-aside />
    <main class="bg-[#1d1f23] text-zinc-400">
      <router-view />
    </main>
    <app-footer />
  </div>
</template>

<script setup lang="ts">
import { onMounted, provide, ref } from 'vue';
import { useRouter } from 'vue-router';
import { injectKeys } from '@/config/constants';
import { useStore } from './composables/use-tauri-store';
import AppFooter from './components/app-footer.vue';
import AppAside from './components/app-aside.vue';

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
