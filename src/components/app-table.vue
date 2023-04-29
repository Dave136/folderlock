<template>
  <div class="flex flex-col">
    <div class="overflow-x-auto sm:mx-0.5">
      <div class="py-2 inline-block min-w-full sm:px-6">
        <div class="overflow-hidden">
          <table class="min-w-full">
            <thead class="bg-transparent border-b border-b-[#282c34]">
              <tr>
                <th
                  scope="col"
                  class="text-sm font-bold text-gray-100 px-6 py-3 text-left"
                  v-for="(header, index) in headers"
                  :key="index"
                >
                  {{ header }}
                </th>
              </tr>
            </thead>
            <tbody>
              <!-- <tr
                class="bg-white border-b transition duration-300 ease-in-out hover:bg-gray-100"
                v-for="item in state.tableInvoices"
                :key="item.id"
              > -->
              <tr
                class="bg-transparent border-b border-b-[#282c34] transition duration-300 ease-in-out hover:bg-[#282c34]"
              >
                <td
                  class="px-6 py-3 whitespace-nowrap text-sm font-bold text-gray-400"
                >
                  Content
                </td>
                <td
                  class="text-sm text-gray-400 font-light px-6 py-3 whitespace-nowrap"
                >
                  Content
                </td>
                <td
                  class="text-sm text-gray-400 font-light px-6 py-3 whitespace-nowrap"
                >
                  Content
                </td>
                <td
                  class="text-sm text-gray-400 font-light px-6 py-3 whitespace-nowrap"
                >
                  Content
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
interface Props {
  headers: string[];
  content: object[];
}

defineProps<Props>();
</script>
