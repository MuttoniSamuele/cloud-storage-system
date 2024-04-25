import { writable } from "svelte/store";
import type File from "../logic/File";
import type Folder from "../logic/Folder";

function initFileMove() {
  const { subscribe, update } = writable<File | Folder | null>(null);

  return {
    subscribe,
    set: (f: File | Folder) => update(() => f),
    cancel: () => update(() => null),
  };
}

export const fileMove = initFileMove();
