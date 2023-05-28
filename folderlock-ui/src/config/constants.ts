import { InjectionKey, Ref } from 'vue';
import tauriConfigJson from '../../src-tauri/tauri.conf.json';
import packageJson from '../../package.json';
import { AppContext } from '@/composables/use-app-setup';

type Context = InjectionKey<Ref<AppContext>>;
type RootPassword = InjectionKey<Ref<boolean>>;

export const DEFAULT_STORE_NAME = 'data.dat';
export const APP_NAME = tauriConfigJson.package.productName;
export const VERSION = packageJson.version;

export const injectKeys = Object.freeze({
  context: Symbol('appContext') as Context,
  rootPassword: Symbol('rootPassword') as RootPassword,
});
