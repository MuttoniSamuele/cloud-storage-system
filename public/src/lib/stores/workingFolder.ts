import { writable } from "svelte/store";

function initWorkingFolder() {
  const { subscribe, set } = writable<string | null>(null);

  return {
    subscribe,
    change: (folder: string | null): void => set(folder),
  };
}

export const workingFolder = initWorkingFolder();
