<script lang="ts">
  export let type: 'button' | 'submit' = 'button';
  export let classname = '';

  export { classname as class };
</script>

<button
  {type}
  class="px-5 border border-zinc-500 py-1 rounded-md transition text-sm text-zinc-500 hover:border-zinc-400 hover:text-zinc-400 {classname} {$$slots.left ||
  $$slots.right
    ? 'flex items-center gap-1'
    : ''}"
  on:click
>
  <slot name="left" />
  <slot />
  <slot name="right" />
</button>
