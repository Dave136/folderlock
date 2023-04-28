import { DEFAULT_STORE_NAME } from '@/config/constants';
import { Store } from 'tauri-plugin-store-api';
import { onMounted, ref, watch } from 'vue';

type IndexType = {
  [key: string]: any;
};

const stores: IndexType = {};
const SAVE_DELAY = 500;

export function getTauriStore(filename: string): Store {
  if (!(filename in stores)) stores[filename] = new Store(filename);
  return stores[filename];
}

export function useTauriStore(
  key: string,
  defaultValue: object | string,
  storeName = DEFAULT_STORE_NAME,
) {
  // storeName -> Is a path that is relative to AppData if not absolute
  const state = ref(defaultValue);
  const loading = ref(true);
  const store = getTauriStore(storeName);
  const timeout = ref<any>(null);

  const setStore = async () => {
    // let allow = true;

    // try {
    //   const value = await store.get(key);
    //   if (value == null) throw '';
    //   if (allow) state.value = value;
    // } catch (error) {
    //   console.log(error);
    //   await store.set(key, defaultValue);
    //   timeout.value = setTimeout(() => store.save(), SAVE_DELAY);

    //   if (allow) loading.value = false;
    // }

    // allow = false;
    let allow = true;
    loading.value = true;
    store
      .get(key)
      .then((value) => {
        console.log('First THEN');
        if (value == null) throw '';
        if (allow) state.value = value;
      })
      .catch(() => {
        console.log('First catch');

        store.set(key, defaultValue).then(() => {
          console.log(key, defaultValue);
          timeout.value = setTimeout(() => store.save(), SAVE_DELAY);
        });
      })
      .then(() => {
        console.log('last THEN');

        if (allow) loading.value = false;
      });

    allow = false;
  };

  const remove = () => {
    store.reset();
  };

  watch(
    state,
    () => {
      // setStore();
      if (!loading.value) {
        console.log('WATCH inside IF');
        clearTimeout(timeout.value);
        store.set(key, state).then(() => {
          timeout.value = setTimeout(() => {
            store.save();
            console.log('saved: ', state.value);
          }, SAVE_DELAY);
        });
      }
    },
    { immediate: true },
  );

  // console.log(state.value);

  onMounted(() => {
    setStore();
  });

  return {
    state,
    loading,
    remove,
  };
}

export function useStore(key: string, storeName = DEFAULT_STORE_NAME) {
  const store = getTauriStore(storeName);
  return {
    get() {
      return store.get(key);
    },
    set(value: any) {
      store.set(key, value);
      store.save();
    },
    remove() {
      store.clear();
      store.reset();
    },
  };
}
