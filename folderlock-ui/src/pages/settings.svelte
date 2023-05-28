<script lang="ts" setup>
  import ViewWrapper from '@/lib/components/app/ViewWrapper.svelte';
  import ActionsWrapper from '@/lib/components/app/ActionsWrapper.svelte';
  import Button from '@/lib/components/Button.svelte';
  import Checkbox from '@/lib/components/Checkbox.svelte';

  const config = {
    minimizeToTray: false,
    checkUpdates: false,
    displayContextMenu: true,
    clearHistory: false,
  };
</script>

<ViewWrapper>
  <div class="flex flex-col flex-1 gap-4">
    <h2 class="text-2xl">App Settings</h2>
    <form class="mt-4">
      <div class="flex flex-col gap-3 px-6">
        <div class="flex items-center gap-2">
          <Checkbox value={config.minimizeToTray} label="Minimize to tray" />
        </div>
        <div class="flex items-center gap-2">
          <Checkbox value={config.checkUpdates} label="Check for updates" />
        </div>
        <div class="flex items-center gap-2">
          <Checkbox
            value={config.displayContextMenu}
            label="Display in context menu"
          />
        </div>
        <div class="flex items-center gap-2">
          <Checkbox
            value={config.clearHistory}
            label="Clear history of File Explorer when closing"
          />
        </div>
        <div>
          <Button>Change login password</Button>
        </div>
      </div>
    </form>
  </div>
  <ActionsWrapper>
    <Button>Save</Button>
  </ActionsWrapper>
</ViewWrapper>
