<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Summary } from "$lib/models/wanikani";
  import { formatHour, firstCount, remainingTotal } from "$lib/models/wanikani";

  // Kick off the Tauri call and use the #await block in markup
  const summaryPromise: Promise<Summary> = invoke(
    "get_summary"
  ) as Promise<Summary>;

</script>

  <h1>WaniKani Summary</h1>

  <p>
    Below are quick counts pulled from your WaniKani summary (first-item counts
    shown as +x, remaining items added as total).
  </p>

  {#await summaryPromise}
    <p>Loading summary…</p>
  {:then summary}
    <section class="cards">
      <article class="card">
        <h2>Lessons</h2>
        {#if summary.lessons && summary.lessons.length > 0}
          <p class="big">+{firstCount(summary.lessons)}</p>
          <p>available at {formatHour(summary.lessons[0].available_at)}</p>

          {#if summary.lessons.length > 1}
            <div class="hour-list">
              {#each summary.lessons.slice(1) as l}
                <span class="hour-item">+{l.subject_ids.length} @ {formatHour(l.available_at)}</span>
              {/each}
            </div>
          {/if}

          <p class="muted">total (remaining) = {remainingTotal(summary.lessons)}</p>
        {:else}
          <p class="big">0</p>
          <p class="muted">no lessons</p>
        {/if}
      </article>

      <a class="card" href="/reviews">
        <h2>Reviews</h2>
        {#if summary.reviews && summary.reviews.length > 0}
          <p class="big">{firstCount(summary.reviews)}</p>
          <p>available at {formatHour(summary.reviews[0].available_at)}</p>
        {:else}
          <p class="big">0</p>
          <p class="muted">no reviews</p>
        {/if}
      </a>
      
      <article class="card">
        <h2>Upcoming Reviews</h2>
        {#if summary.reviews && summary.reviews.length > 1}
          <div class="hour-list">
            {#each summary.reviews.slice(1) as r}
              <span class="hour-item">{r.subject_ids.length} @ {formatHour(r.available_at)}</span>
            {/each}
          </div>
          <p class="muted">total (remaining) = {remainingTotal(summary.reviews)}</p>
        {:else}
          <p class="muted">no upcoming reviews</p>
        {/if}
      </article>
    </section>
  {:catch err}
    <p class="error">Failed to load summary: {String(err)}</p>
  {/await}

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }
  }

  /* Cards and small helpers */
  .cards {
    display: grid;
    place-items: center;
    grid-auto-flow: row;
    gap: 1rem;
    margin-top: 1.25rem;
    flex-wrap: wrap;
  }

  .card {
    background: #ffffff;
    padding: 1rem 1.25rem;
    border-radius: 10px;
    min-width: 180px;
    box-shadow: 0 6px 14px rgba(16,24,40,0.08);
    text-align: left;
    width: 75%;
  }

  .big {
    font-size: 2rem;
    font-weight: 700;
    margin: 0.25rem 0;
  }

  .muted {
    color: #6b7280;
    font-size: 0.95rem;
  }

  .hour-list {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin: 0.5rem 0;
  }

  .hour-item {
    background: #f3f4f6;
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    font-weight: 600;
    font-size: 0.95rem;
  }

  .error {
    color: #b91c1c;
  }
</style>
