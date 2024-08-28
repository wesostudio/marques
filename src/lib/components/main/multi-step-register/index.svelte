<script>
  import { goto } from "$app/navigation";
  import Button from "../../ui/button/button.svelte";
  import Presentation from "./steps/Presentation.svelte";
  import CommerceInfo from "./steps/CommerceInfo.svelte";
  import TypeOfCommerce from "./steps/TypeOfCommerce.svelte";
  import OwnerDetails from "./steps/OwnerDetails.svelte";
  import PhoneInput from "./steps/PhoneInput.svelte";
  import LocationSelector from "./steps/LocationSelector.svelte";
  import AddServiceForm from "./steps/AddServiceForm.svelte";
  import TeamFiller from "./steps/TeamFiller.svelte";

  let currentStep = 0;

  function handleSteps() {
    if (currentStep === 7) {
      // Ver donde manejar la subida a la db
      // y guardar token de sesion en localstorage
      // cargar data del commerce en localstorage
      goto("/dashboard");
      return;
    }

    currentStep++;
  }
</script>

<main class="h-screen grid place-items-center">
  <section class="flex flex-col gap-8">
    {#if currentStep === 0}
      <Presentation />
    {:else if currentStep === 1}
      <CommerceInfo />
    {:else if currentStep === 2}
      <TypeOfCommerce />
    {:else if currentStep === 3}
      <OwnerDetails />
    {:else if currentStep === 4}
      <PhoneInput />
    {:else if currentStep === 5}
      <LocationSelector />
    {:else if currentStep === 6}
      <AddServiceForm />
    {:else if currentStep === 7}
      <TeamFiller />
    {/if}
    <section class="">
      <Button on:click={handleSteps}>Siguiente</Button>
    </section>
  </section>
</main>
