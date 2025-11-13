import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";

export const load = (async () => {
  const apiKey = invoke("get_api_key")
    .then((key) => key as string | null)
    .catch(() => null);

  return {
    apiKey: await apiKey,
  };
}) satisfies PageLoad;
