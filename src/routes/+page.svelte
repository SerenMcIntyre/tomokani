<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Summary } from "$lib/models/wanikani";
  import { formatHour, firstCount, remainingTotal } from "$lib/models/wanikani";

  // Kick off the Tauri call and use the #await block in markup
  const summaryPromise: Promise<Summary> = invoke(
    "get_summary"
  ) as Promise<Summary>;

  const get_user = async () => {
    try {
      const data = await invoke("get_user");
      console.log("WaniKani User:", data);
      alert(JSON.stringify(data));
    } catch (err) {
      console.error("get_user failed", err);
      alert("get_user failed: " + JSON.stringify(err));
    }
  };
</script>

  <h1>WaniKani Summary</h1>

  <p>
    Below are quick counts pulled from your WaniKani summary (first-item counts
    shown as +x, remaining items added as total).
  </p>
  <!-- 
  <form class="row" onsubmit={() => set_api_key(key)}>
    <input id="greet-input" placeholder="Set api key" bind:value={key} />
    <button type="submit">Set API Key</button>
  </form> -->
  <p></p>
  <button type="button" on:click={() => get_user()}>Get WaniKani User</button>

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

      <article class="card">
        <h2>Reviews</h2>
        {#if summary.reviews && summary.reviews.length > 0}
          <p class="big">{firstCount(summary.reviews)}</p>
          <p>available at {formatHour(summary.reviews[0].available_at)}</p>
        {:else}
          <p class="big">0</p>
          <p class="muted">no reviews</p>
        {/if}
      </article>
      
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
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte-kit:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

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

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
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

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
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
