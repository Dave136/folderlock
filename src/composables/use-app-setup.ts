import { Ref, onMounted, ref } from 'vue';
import * as tauriPath from '@tauri-apps/api/path';
import * as fs from '@tauri-apps/api/fs';
import * as os from '@tauri-apps/api/os';
import { APP_NAME } from '@/config/constants';

export interface AppContext {
  loading: boolean;
  downloads: undefined | string;
  documents: undefined | string;
  appDocuments: undefined | string;
  osType: undefined | string;
  fileSep: string;
}

type UseAppSetupReturn = Ref<AppContext>;

export default function useAppSetup(): UseAppSetupReturn {
  const appContext = ref<AppContext>({
    loading: false,
    downloads: undefined,
    documents: undefined,
    appDocuments: undefined,
    osType: undefined,
    fileSep: '/',
  });

  onMounted(async () => {
    appContext.value.loading = true;
    try {
      const downloadDir = await tauriPath.downloadDir();
      const documents = await tauriPath.documentDir();
      const osType = await os.type();
      const fileSep = osType === 'Windows_NT' ? '\\' : '/';
      const appDocuments = `${documents}${APP_NAME}`;

      appContext.value.downloads = downloadDir;
      appContext.value.documents = documents;
      appContext.value.appDocuments = appDocuments;
      appContext.value.osType = osType;
      appContext.value.fileSep = fileSep;

      await fs.createDir(APP_NAME, {
        dir: fs.BaseDirectory.Cache,
        recursive: true,
      });
    } catch (error) {
      console.error('useAppSetup -> ', error);
    } finally {
      appContext.value.loading = false;
    }
  });

  return appContext;
}
