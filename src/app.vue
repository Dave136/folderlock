<template>
  <div
    class="h-screen bg-[#282c34] text-zinc-100 grid grid-rows-[1fr_1.5rem]"
    :class="existRootPassword ? 'grid-cols-[3rem_1fr]' : 'grid-cols-1'"
  >
    <app-aside v-if="existRootPassword" />
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
import AppFooter from './components/app-footer.vue';
import AppAside from './components/app-aside.vue';
import useAppSetup from './composables/use-app-setup';
import { invoke } from '@tauri-apps/api/tauri';

const router = useRouter();
const showAside = ref(false);
const existRootPassword = ref(false);

const context = useAppSetup();

provide(injectKeys.context, context);
provide(injectKeys.rootPassword, existRootPassword);

onMounted(async () => {
  try {
    await invoke('has_app_password');
    showAside.value = true;
    existRootPassword.value = true;
    return router.push('/');
  } catch (error) {
    existRootPassword.value = false;
    return router.push('/auth');
  }
});
</script>
