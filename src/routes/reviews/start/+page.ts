import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";
import { reviewSession } from "../review-session.svelte";
import * as v from "valibot";
import { SubjectSchema } from "$lib/models/subject";

export const load = (async () => {
  const subjects = await invoke("get_subjects_by_ids", {
    ids: reviewSession.reviewIds,
  });
  return {
    subjects: v.parse(v.array(SubjectSchema), subjects),
  };
}) satisfies PageLoad;
