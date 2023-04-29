<template>
  <view-wrapper>
    <blank-message message="Encrypt files to mantain it secure">
      <template #icon>
        <ph-file-lock :size="100" weight="fill" />
      </template>
    </blank-message>
    <actions-wrapper>
      <app-button>Create</app-button>
      <app-button>Open</app-button>
    </actions-wrapper>
  </view-wrapper>
</template>

<script lang="ts" setup>
import { PhFileLock } from '@phosphor-icons/vue';
import ViewWrapper from '@/components/view-wrapper.vue';
import BlankMessage from '@/components/blank-message.vue';
import ActionsWrapper from '@/components/actions-wrapper.vue';
import AppButton from '@/components/app-button.vue';
</script>
