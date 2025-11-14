<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import type { PageData } from "./$types";
  import { reviewSession } from "./review-session.svelte";

  let { data }: { data: PageData } = $props();

  const availReviews = data.summary?.reviews.at(0)?.subject_ids || [];
  const reviewCount = $derived(availReviews.length);

  onMount(() => {
    reviewSession.reset();
  });

  // make a dropdown list of number of reviews to do, from [1, 3, 5, 10, 15, multiples of 5, or "all"]
  const options = $derived.by(() => {
    const opts = [1, 3, 5, 10, 15].filter((n) => n < reviewCount);
    const total = availReviews.length;
    for (let i = 20; i <= total; i += 5) {
      opts.push(i);
    }
    if (total > 0) {
      opts.push(reviewCount);
    }
    return opts;
  });

  const onsubmit = (event: Event) => {
    event.preventDefault();
    const form = event.target as HTMLFormElement;
    const select = form.querySelector("select") as HTMLSelectElement;
    const value = select.value;
    let count: number;
    if (value === reviewCount.toString()) {
      count = availReviews.length;
    } else {
      count = parseInt(value);
    }

    reviewSession.reviewIds = // randomly select 'count' items from availReviews
      availReviews
        .map((id) => ({ id, sort: Math.random() }))
        .sort((a, b) => a.sort - b.sort)
        .slice(0, count)
        .map((obj) => obj.id);

    goto(`/reviews/start`);
  };
</script>

<form {onsubmit}>
  <select>
    {#each options as option}
      <option value={option}
        >{option === reviewCount ? `All (${reviewCount})` : option}</option
      >
    {/each}
  </select>
  <button type="submit">Start</button>
</form>
