<script>
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import Textarea from "$lib/components/ui/textarea/textarea.svelte";
</script>

<h2 class="font-semibold text-4xl">Tu comercio</h2>

<div>
  <Label for="" class="block text-sm font-semibold dark:text-gray-300"
    >Nombre</Label
  >

  <Input
    type="text"
    placeholder="Mi comercio"
    class="block mt-2 w-full placeholder-gray-400/70 dark:placeholder-gray-500 rounded-lg border border-gray-200 bg-white px-5 py-2.5 text-gray-700 focus:border-blue-400 focus:outline-none focus:ring focus:ring-blue-300 focus:ring-opacity-40 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-300 dark:focus:border-blue-300"
  />
</div>
<div>
  <Label for="" class="block text-sm text-gray-500 dark:text-gray-300"
    >Descripcion</Label
  >

  <Textarea
    placeholder="Descripcion de tu servicio"
    class="block mt-2 w-full placeholder-gray-400/70 dark:placeholder-gray-500 rounded-lg border border-gray-200 bg-white px-5 py-2.5 text-gray-700 focus:border-blue-400 focus:outline-none focus:ring focus:ring-blue-300 focus:ring-opacity-40 dark:border-gray-600 dark:bg-gray-900 dark:text-gray-300 dark:focus:border-blue-300"
  />
</div>
