import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';
import HideView from '@/views/hide.vue';
import { useStore } from '@/composables/use-tauri-store';

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
];

const router = createRouter({
  routes,
  history: createWebHistory(),
});

router.beforeEach(async (to, from) => {
  const store = useStore('password');
  const canAccess = (await store.get()) as string;

  if (!canAccess.length && to.path !== '/auth') {
    return '/auth';
  }
});

export default router;
