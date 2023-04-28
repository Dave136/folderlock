import { InjectionKey, Ref } from 'vue';

type HasApplicationPassword = InjectionKey<Ref<boolean>>;

export const DEFAULT_STORE_NAME = 'data.dat';

export const injectKeys = Object.freeze({
  hasApplicationPassword: Symbol(
    'hasApplicationPassword',
  ) as HasApplicationPassword,
});
