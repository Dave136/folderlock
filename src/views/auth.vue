<template>
  <section class="max-w-sm mx-auto flex justify-center items-center h-full">
    <form
      class="bg-green-800 mt-36 w-full p-8 rounded-md bg-zinc-700"
      @keyup.enter="onSubmit"
    >
      <h2 class="text-2xl text-center">Set app password</h2>
      <div class="flex flex-col mt-8">
        <label for="password">Password</label>
        <input
          name="password"
          type="password"
          class="p-2 mt-2 rounded-md bg-zinc-600 ring ring-transparent"
          v-model="password"
        />
      </div>
      <div class="flex flex-col mt-8">
        <label for="password">Repeat password</label>
        <input
          name="repeatPassword"
          type="password"
          class="p-2 mt-2 rounded-md bg-zinc-600 ring ring-transparent"
          v-model="repeatPassword"
        />
      </div>

      <p
        class="text-xs mt-1 text-red-400"
        v-show="meta.touched && errorMessage"
      >
        {{ errorMessage }}
      </p>

      <div class="flex flex-col mt-8">
        <button
          type="button"
          class="!bg-cyan-700 p-4 rounded-md transition-colors hover:!bg-cyan-800"
          @click.prevent="onSubmit"
        >
          Save
        </button>
      </div>
    </form>
  </section>
</template>

<script lang="ts" setup>
import { useForm, useField } from 'vee-validate';
import { object, string, ref } from 'yup';
import { useRouter } from 'vue-router';
import { useStore } from '@/composables/use-tauri-store';

const router = useRouter();
const store = useStore('password');
// remove();
store.remove();

const schema = object({
  password: string().required('Field required'),
  repeatPassword: string()
    .required('Field required')
    .oneOf([ref('password')], 'Passwords not match'),
});

const { useFieldModel, handleSubmit } = useForm({
  validationSchema: schema,
});

const { errorMessage, meta } = useField('repeatPassword');

const [password, repeatPassword] = useFieldModel([
  'password',
  'repeatPassword',
]);

const onSubmit = handleSubmit(async (data) => {
  store.set(data.password);
  router.push('/');
});
</script>
