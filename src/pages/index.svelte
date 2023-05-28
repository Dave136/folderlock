<script lang="ts">
  import { Pencil, TrashSimple, File } from 'phosphor-svelte';
  import ViewWrapper from '$lib/components/app/ViewWrapper.svelte';
  import BlankMessage from '@/lib/components/app/BlankMessage.svelte';
  import Content from '@/lib/components/app/Content.svelte';
  import TableWrapper from '$lib/components/table/TableWrapper.svelte';
  import TableHead from '$lib/components/table/TableHead.svelte';
  import TableTh from '$lib/components/table/TableTh.svelte';
  import TableBody from '$lib/components/table/TableBody.svelte';
  import TableTd from '$lib/components/table/TableTd.svelte';
  import TableTr from '$lib/components/table/TableTr.svelte';
  import ActionsWrapper from '@/lib/components/app/ActionsWrapper.svelte';
  import Button from '$lib/components/Button.svelte';
  import { dropZone } from '@/lib/actions/drop-zone';

  let dropdown = {
    id: '',
    active: false,
  };

  let isOverDropZone = false;

  const listHeaders = ['Name', 'Lock', 'Status', 'Action'];

  // const fakeData = [
  //   {
  //     id: '1',
  //     name: 'folder',
  //     lock: false,
  //     hidden: false,
  //   },
  // ];

  const fakeData: string[] = [];

  function openDropdown(id: string) {
    dropdown.id = id;
    dropdown.active = true;
  }

  function dropdownReset() {
    dropdown = {
      id: '',
      active: false,
    };
  }
</script>

<ViewWrapper>
  {#if !fakeData.length}
    <div
      class="h-full flex justify-center items-center"
      use:dropZone
      on:dragEnter={() => {
        isOverDropZone = true;
      }}
      on:dragLeave={() => {
        isOverDropZone = false;
      }}
    >
      <BlankMessage
        class="h-full flex justify-center {isOverDropZone
          ? 'border border-dashed border-blue-6'
          : ''}"
      >
        <slot name="icon">
          <File size={100} weight="fill" />
        </slot>
        Drag and drop files or folders to the blank area to hide
      </BlankMessage>
    </div>
  {:else}
    <Content>
      <div class="w-full h-full">
        <TableWrapper>
          <TableHead>
            {#each listHeaders as header}
              <TableTh>{header}</TableTh>
            {/each}
          </TableHead>
          <TableBody>
            {#each fakeData as item}
              <TableTr>
                <TableTd>{item.name}</TableTd>
                <TableTd>{item.lock ? 'Yes' : 'No'}</TableTd>
                <TableTd>{item.hidden ? 'Hide' : 'Show'}</TableTd>
                <TableTd class="relative !p-0 flex">
                  <button
                    class="flex relative z-10 block p-2 text-gray-300 bg-dark-300 border border-transparent rounded-md dark:text-white focus:border-blue-500 focus:ring-opacity-40 dark:focus:ring-opacity-40 focus:ring-blue-300 dark:focus:ring-blue-400 focus:ring dark:bg-black dark:bg-opacity-10 focus:outline-none"
                    on:click={() => openDropdown(item.id)}
                  >
                    Options
                    <svg
                      class="-mr-1 h-5 w-5 text-gray-400"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                      aria-hidden="true"
                    >
                      <path
                        fill-rule="evenodd"
                        d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>

                  <!-- Dropdown menu -->
                  {#if dropdown.active && dropdown.id === item.id}
                    <div
                      class="absolute z-20 w-32 py-1 mt-2 origin-top-right bg-dark-300 rounded-md shadow-xl dark:bg-zinc-900"
                      on:click={dropdownReset}
                    >
                      <a
                        href="#"
                        class="flex items-center gap-2 px-4 py-2 text-xs text-gray-300 capitalize transition-colors duration-300 transform dark:text-gray-300 hover:bg-dark-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        <Pencil size={16} color="#ffffff" weight="thin" />
                        Edit
                      </a>
                      <a
                        href="#"
                        class="flex items-center gap-2 px-4 py-2 text-xs text-gray-300 capitalize transition-colors duration-300 transform dark:text-gray-300 hover:bg-dark-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        <TrashSimple size={16} color="#ffffff" weight="thin" />
                        Delete
                      </a>
                      <hr class="mx-4 border border-dark-100" />
                      <a
                        href="#"
                        class="flex items-center gap-2 px-4 py-2 text-xs text-gray-300 capitalize transition-colors duration-300 transform dark:text-gray-300 hover:bg-dark-400 dark:hover:bg-gray-700 dark:hover:text-white"
                      >
                        <!-- <ph-trash-simple
                          size="16"
                          color="#ffffff"
                          weight="thin"
                        /> -->
                        Set password
                      </a>
                    </div>
                  {/if}
                </TableTd>
              </TableTr>
            {/each}
          </TableBody>
        </TableWrapper>
      </div>
    </Content>
  {/if}
  <ActionsWrapper>
    <Button on:click>
      <slot name="left">Icon</slot>
      Hide File
    </Button>
    <Button on:click>
      <slot name="left">Icon</slot>
      Hide Folder
    </Button>
  </ActionsWrapper>
</ViewWrapper>
