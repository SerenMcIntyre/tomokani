import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import { reviewSession } from "../review-session.svelte";

export const load = (async () => {
  const subjects = await invoke("get_subjects_by_ids", {
    ids: reviewSession.reviewIds,
  });
  return {
    subjects,
  };
}) satisfies PageLoad;
