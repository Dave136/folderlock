<script lang="ts">
  let classname = '';
  export { classname as class };
</script>

<section class="p-4 flex flex-col h-full {classname}">
  <slot />
</section>
