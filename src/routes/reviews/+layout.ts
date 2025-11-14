import type { Summary } from "$lib/models/wanikani";
import { invoke } from "@tauri-apps/api/core";
import type { LayoutLoad } from "./$types";

export const load = (async () => {
  const summary: Summary = await invoke("get_summary");

  return {
    summary,
  };
}) satisfies LayoutLoad;
