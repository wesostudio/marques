<script lang="ts">
  import { cn } from "$lib/utils.js";

  let className: string | undefined | null = undefined;
  export { className as class };
</script>

<nav class={cn("flex items-center space-x-4 lg:space-x-6", className)}>
  <a
    href="/dashboard"
    class="text-sm font-medium transition-colors hover:text-primary"
  >
    Inicio
  </a>

  <a
    href="/dashboard/clients"
    class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
  >
    Clientes
  </a>
  <a
    href="/dashboard/products"
    class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
  >
    Productos
  </a>
  <a
    href="/dashboard/services"
    class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
  >
    Servicios
  </a>
  <a
    href="/dashboard/settings"
    class="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
  >
    Ajustes
  </a>
</nav>
