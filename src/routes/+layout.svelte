<script lang="ts">
  import type { Snippet } from "svelte";
  import type { LayoutData } from "./$types";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";

  let { data, children }: { data: LayoutData; children: Snippet } = $props();
  $effect(() => {
    alert(`Layout data: ${JSON.stringify(data.user)}`);
    if (page.route.id !== "/settings" && !data.configValid) {
        alert("Please set your WaniKani API Key in Settings before continuing.");
        goto("/settings");
    }
  });

</script>

<nav>
  <a href="/">Home</a> |
  <a href="/settings">Settings</a>
</nav>
<main>
  {@render children()}
</main>
