import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';
import HomeView from '@/views/home.vue';
import { useStore } from '@/composables/use-tauri-store';

const loadRoute = async (route: string) => import(`@/views/${route}.vue`);

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: HomeView,
  },
  {
    path: '/auth',
    component: loadRoute('auth'),
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
