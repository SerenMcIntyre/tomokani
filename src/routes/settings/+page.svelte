<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PageData } from "./$types";
  import * as v from "valibot";

  let { data }: { data: PageData } = $props();

  const settingsSchema = v.object({
    apiKey: v.union([v.string()]),
  });

  let settings = $state({
    apiKey: data.apiKey || "",
  });

  let validation = $derived(v.safeParse(settingsSchema, settings));

  const onsubmit = (event: Event) => {
    event.preventDefault();
    if (data.apiKey !== settings.apiKey) {
      alert("API Key updated.");
      invoke("set_api_key", { apiKey: settings.apiKey }).then((msg) => {
        alert(msg as string);
      });
    }
  };
</script>

<form {onsubmit}>
  <label for="apiKey">WaniKani API Key:</label>
  <input
    id="apiKey"
    type="text"
    bind:value={settings.apiKey}
    placeholder="Enter your WaniKani API Key"
  />
  <button type="submit" disabled={!validation.success}>Save</button>
</form>
