<template>
  <div class="flex flex-col">
    <div class="overflow-x-auto sm:mx-0.5">
      <div class="py-2 inline-block min-w-full sm:px-6">
        <div class="overflow-hidden">
          <table class="min-w-full">
            <slot />
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
