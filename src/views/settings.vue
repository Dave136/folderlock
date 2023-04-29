<template>
  <view-wrapper>
    <div class="flex flex-col flex-1 gap-4">
      <h2 class="text-2xl">App Settings</h2>
      <form class="mt-4">
        <div class="flex flex-col gap-3 px-6">
          <div class="flex items-center gap-2">
            <app-checkbox
              v-model="config.minimizeToTray"
              label="Minimize to tray"
            />
          </div>
          <div class="flex items-center gap-2">
            <app-checkbox
              v-model="config.checkUpdates"
              label="Check for updates"
            />
          </div>
          <div class="flex items-center gap-2">
            <app-checkbox
              v-model="config.displayContextMenu"
              label="Display in context menu"
            />
          </div>
          <div class="flex items-center gap-2">
            <app-checkbox
              v-model="config.clearHistory"
              label="Clear history of File Explorer when closing"
            />
          </div>
          <div>
            <app-button>Change login password</app-button>
          </div>
        </div>
      </form>
    </div>
    <actions-wrapper>
      <app-button>Save</app-button>
    </actions-wrapper>
  </view-wrapper>
</template>

<script lang="ts" setup>
import ActionsWrapper from '@/components/actions-wrapper.vue';
import AppButton from '@/components/app-button.vue';
import AppCheckbox from '@/components/app-checkbox.vue';
import ViewWrapper from '@/components/view-wrapper.vue';
import { ref } from 'vue';

const config = ref({
  minimizeToTray: false,
  checkUpdates: false,
  displayContextMenu: true,
  clearHistory: false,
});
</script>
