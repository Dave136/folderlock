<script lang="ts">
  let classname = '';

  export { classname as class };
</script>

<div
  class="flex flex-col justify-center items-center flex-1 gap-4 {classname}"
  on:mouseenter
  on:mouseleave
>
  <slot name="icon" />
  <slot />
</div>
