<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="flex gap-3">
    <input
      class="rounded-lg border border-transparent px-5 py-2 font-medium text-zinc-700 bg-white transition shadow shadow-md"
      v-model="name"
      placeholder="Enter a name..."
    />
    <button
      class="rounded-lg border border-transparent px-5 py-2 font-medium !bg-zinc-800 transition shadow shadow-md cursor-pointer !text-zinc-100 hover:border-blue-500"
      type="button"
      @click="greet()"
    >
      Greet
    </button>
  </div>

  <p class="mt-8 p-2 border-b">{{ greetMsg }}</p>
</template>
