<template>
  <view-wrapper>
    <blank-message
      message="Drag and drop files or folders to the blank area to hide"
      ref="dropZoneRef"
      :class="[
        isOverDropZone || false ? 'border border-dashed border-blue-600' : '',
      ]"
      v-if="!hideList.length"
    >
      <template #icon>
        <ph-file :size="100" weight="fill" />
      </template>
    </blank-message>
    <app-content v-else>
      <div class="w-full h-full">
        <table-wrapper>
          <table-head>
            <table-th v-for="(header, index) in listHeaders" :key="index">
              {{ header }}
            </table-th>
          </table-head>
          <table-body>
            <table-tr v-for="(item, index) in hideList" :key="index">
              <table-td>
                {{ item.name }}
              </table-td>
              <table-td>
                {{ item.lock ? 'Si' : 'No' }}
              </table-td>
              <table-td>
                {{ item.hidden ? 'Hidden' : 'Show' }}
              </table-td>
              <table-td class="relative !p-0 flex">
                <div class="top-2 right-10 inline-block">
                  <!-- Dropdown toggle button -->
                  <button
                    class="flex relative z-10 block p-2 text-gray-300 bg-dark-300 border border-transparent rounded-md dark:text-white focus:border-blue-500 focus:ring-opacity-40 dark:focus:ring-opacity-40 focus:ring-blue-300 dark:focus:ring-blue-400 focus:ring dark:bg-black dark:bg-opacity-10 focus:outline-none"
                    @click="openDropdown(item.id)"
                  >
                    Options
                    <svg
                      class="-mr-1 h-5 w-5 text-gray-400"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                      aria-hidden="true"
                    >
                      <path
                        fill-rule="evenodd"
                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>

                  <!-- Dropdown menu -->
                  <transition name="dropdown" mode="out-in">
                    <div
                      v-show="dropdown.active && dropdown.id === item.id"
                      class="absolute z-20 w-32 py-1 mt-2 origin-top-right bg-dark-300 rounded-md shadow-xl dark:bg-zinc-900"
                      @click="dropdownReset"
                    >
                      <a
                        href="#"
                        class="flex items-center gap-2 px-4 py-2 text-xs text-gray-300 capitalize transition-colors duration-300 transform dark:text-gray-300 hover:bg-dark-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        <ph-pencil :size="16" color="#ffffff" weight="thin" />
                        Edit
                      </a>
                      <a
                        href="#"
                        class="flex items-center gap-2 px-4 py-2 text-xs text-gray-300 capitalize transition-colors duration-300 transform dark:text-gray-300 hover:bg-dark-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        <ph-trash-simple
                          size="16"
                          color="#ffffff"
                          weight="thin"
                        />
                        Delete
                      </a>
                      <hr class="mx-4 border border-dark-100" />
                      <a
                        href="#"
                        class="flex items-center gap-2 px-4 py-2 text-xs text-gray-300 capitalize transition-colors duration-300 transform dark:text-gray-300 hover:bg-dark-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        <ph-trash-simple
                          size="16"
                          color="#ffffff"
                          weight="thin"
                        />
                        Set password
                      </a>
                    </div>
                  </transition>
                </div>
              </table-td>
            </table-tr>
          </table-body>
        </table-wrapper>
      </div>
    </app-content>
    <actions-wrapper>
      <app-button @click="openDownloadDir">
        <template #left>
          <ph-file :size="16" weight="fill" />
        </template>
        Hide File</app-button
      >
      <app-button>
        <template #left>
          <ph-folder :size="16" weight="fill" />
        </template>
        Hide Folder</app-button
      >
    </actions-wrapper>
  </view-wrapper>
</template>

<script lang="ts" setup>
import { useDropZone } from '@vueuse/core';
import { inject, reactive, ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { PhFile, PhFolder } from '@phosphor-icons/vue';
import AppButton from '@/components/app-button.vue';
import ViewWrapper from '@/components/view-wrapper.vue';
import ActionsWrapper from '@/components/actions-wrapper.vue';
import BlankMessage from '@/components/blank-message.vue';
import AppContent from '@/components/app-content.vue';
import { injectKeys } from '@/config/constants';
import { shell } from '@tauri-apps/api';
import TableWrapper from '@/components/table-wrapper.vue';
import TableHead from '@/components/table-head.vue';
import TableTh from '@/components/table-th.vue';
import TableBody from '@/components/table-body.vue';
import TableTd from '@/components/table-td.vue';
import TableTr from '@/components/table-tr.vue';
import { invoke } from '@tauri-apps/api/tauri';

interface List {
  id: string;
  name: string;
  lock: boolean;
  hidden: boolean;
}

const dropZoneRef = ref<HTMLElement | null>(null);
const { isOverDropZone } = useDropZone(dropZoneRef);
const UNLISTEN_DELAY = 2000;
const context = inject(injectKeys.context);

const dropdown = reactive({
  id: '',
  active: false,
});

const hideList = ref<List[]>([]);

const listHeaders: string[] = ['Name', 'Lock', 'Status', 'Action'];

// const storeName = `${context?.value.documents}${APP_NAME}/data.dat`;

const openDownloadDir = async () => {
  await shell.open(context?.value.downloads as string);
};

const openDropdown = (id: string) => {
  dropdown.id = id;
  dropdown.active = !dropdown.active;
};

const dropdownReset = () => {
  dropdown.id = '';
  dropdown.active = false;
};

watch(isOverDropZone, async (value: boolean) => {
  if (!value) return;

  const unlisten = await listen('tauri://file-drop', async (event) => {
    const files = event.payload as string[];

    await invoke('hide_file', { path: files[0] });

    files.forEach((file) => {
      hideList.value.push({
        id: crypto.randomUUID(),
        name: file,
        lock: false,
        hidden: false,
      });
    });
  });

  setTimeout(() => {
    unlisten();
  }, UNLISTEN_DELAY);
});
</script>

<style scoped>
/* we will explain what these classes do next! */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.2s ease-in;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
}
</style>
