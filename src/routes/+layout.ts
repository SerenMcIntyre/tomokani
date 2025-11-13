// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps

import { invoke } from "@tauri-apps/api/core";
import type { LayoutLoad } from "./$types";
import * as v from "valibot";
import { UserSchema } from "$lib/models/user";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

export const load = (async () => {
  const apiKey = await invoke("get_api_key")
    .then((key) => key as string | null)
    .catch(() => null);

  const user = await invoke("get_user")
    .then((info) => v.parse(UserSchema, info))
    .catch(() => null);

  return {
    apiKey,
    user,
    configValid: user !== null,
  };
}) satisfies LayoutLoad;