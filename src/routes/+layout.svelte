<script lang="ts">
  import type { Snippet } from "svelte";
  import type { LayoutData } from "./$types";

  let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<nav>
  <a href="/">Home</a> |
  <a href="/settings">Settings</a>
</nav>
<main>
  {@render children()}
</main>
