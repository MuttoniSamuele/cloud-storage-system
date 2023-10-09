import { writable } from "svelte/store";
import type Path from "../logic/Path";

function initWorkingFolder() {
  const { subscribe, set } = writable<Path | null>(null);

  return {
    subscribe,
    change: (path: Path | null): void => set(path),
  };
}

export const workingFolder = initWorkingFolder();
