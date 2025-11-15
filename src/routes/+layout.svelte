<script lang="ts">
  import type { Snippet } from "svelte";
  import "../app.css";
  import type { LayoutData } from "./$types";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";

  let { data, children }: { data: LayoutData; children: Snippet } = $props();
  $effect(() => {
    if (page.route.id !== "/settings" && !data.configValid) {
        goto("/settings");
    }
  });

</script>

<nav class="p-4">
  <a href="/">Home</a> |
  <a href="/settings">Settings</a>
</nav>
<main>
  {@render children()}
</main>
