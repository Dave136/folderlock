import { writable, type Readable } from 'svelte/store';
import * as tauriPath from '@tauri-apps/api/path';
// import * as fs from '@tauri-apps/api/fs';
import * as os from '@tauri-apps/api/os';
import { APP_NAME } from '@/config/constants';

export interface GlobalStore {
  downloads: undefined | string;
  documents: undefined | string;
  appDocuments: undefined | string;
  osType: undefined | string;
  fileSep: string;
}

export interface GlobalStoreMethods {
  init: () => Promise<void>;
}

const initialValues: GlobalStore = {
  downloads: undefined,
  documents: undefined,
  appDocuments: undefined,
  osType: undefined,
  fileSep: '/',
};

export function createGlobalStore() {
  const globalStore = writable<GlobalStore>(initialValues);

  const { subscribe, update } = globalStore;

  const init = async () => {
    const downloadDir = await tauriPath.downloadDir();
    const documents = await tauriPath.documentDir();
    const osType = await os.type();
    const fileSep = osType === 'Windows_NT' ? '\\' : '/';
    const appDocuments = `${documents}${APP_NAME}`;
    // const existPath = await fs.exists(APP_NAME);

    update(() => ({
      downloads: downloadDir,
      documents,
      appDocuments,
      osType,
      fileSep,
    }));
  };

  // if (!existPath) {
  //   await fs.createDir(APP_NAME, {
  //     dir: fs.BaseDirectory.Cache,
  //     recursive: true,
  //   });
  // }

  return {
    subscribe,
    init,
  };
}

const globalStore = createGlobalStore() as unknown as Readable<GlobalStore> &
  GlobalStoreMethods;

export default globalStore;
