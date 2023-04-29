<template>
  <view-wrapper>
    <blank-message
      message="Drag and drop files or folders to the blank area to hide"
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
import { PhFile, PhFolder } from '@phosphor-icons/vue';
import AppButton from '@/components/app-button.vue';
import ViewWrapper from '@/components/view-wrapper.vue';
import ActionsWrapper from '@/components/actions-wrapper.vue';
import BlankMessage from '@/components/blank-message.vue';
</script>
