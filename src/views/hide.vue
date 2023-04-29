<template>
  <view-wrapper>
    <blank-message
      message="Drag and drop files or folders to the blank area to hide"
      ref="dropZoneRef"
      :class="[
        isOverDropZone || false ? 'border border-dashed border-blue-600' : '',
      ]"
      @mouseenter="canListenDrop = true"
      @mouseleave="canListenDrop = false"
    >
      <template #icon>
        <ph-file :size="100" weight="fill" />
      </template>
    </blank-message>
    <actions-wrapper>
      <app-button>
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
import { ref, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { PhFile, PhFolder } from '@phosphor-icons/vue';
import AppButton from '@/components/app-button.vue';
import ViewWrapper from '@/components/view-wrapper.vue';
import ActionsWrapper from '@/components/actions-wrapper.vue';
import BlankMessage from '@/components/blank-message.vue';

const UNLISTEN_DELAY = 2000;

const dropZoneRef = ref<HTMLElement | null>(null);
const canListenDrop = ref<boolean>(false);

const { isOverDropZone } = useDropZone(dropZoneRef);

watch(isOverDropZone, async (value: boolean) => {
  if (!value) return;

  const unlisten = await listen('tauri://file-drop', (event) => {
    console.log(event);
  });

  setTimeout(() => {
    unlisten();
  }, UNLISTEN_DELAY);
});
</script>
