<script lang="ts">
  import { goto } from "$app/navigation";
  import { reviewSession } from "../review-session.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();

  if (!reviewSession.reviewIds || reviewSession.reviewIds.length === 0) {
    goto("/reviews");
  }

  alert(JSON.stringify(data.subjects));
</script>

{#each reviewSession.reviewIds as id}
  <p>Reviewing subject ID: {id}</p>
{/each}
