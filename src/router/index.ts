import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';
import HideView from '@/views/hide.vue';
import { invoke } from '@tauri-apps/api';

const loadRoute = async (route: string) => import(`@/views/${route}.vue`);

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: HideView,
  },
  {
    path: '/auth',
    component: loadRoute('auth'),
  },
  {
    path: '/encrypt',
    component: loadRoute('encrypt'),
  },
  {
    path: '/settings',
    component: loadRoute('settings'),
  },
];

const router = createRouter({
  routes,
  history: createWebHistory(),
});

router.beforeEach(async (to, from) => {
  try {
    const canAccess = await invoke('has_app_password');

    console.log({ canAccess });

    if (!canAccess && to.path !== '/auth') {
      return '/auth';
    }
  } catch (error) {
    console.log(error);
  }
});

export default router;
