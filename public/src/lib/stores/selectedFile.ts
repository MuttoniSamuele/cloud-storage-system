import { writable } from "svelte/store";
import type Folder from "../logic/Folder";
import type File from "../logic/File";

function initSelectedFiles() {
  const selectedFiles = new Set<File | Folder>();
  const { subscribe, update } = writable<Set<File | Folder>>(selectedFiles);

  return {
    subscribe,
    add: (f: File | Folder): void => update((s) => {
      s.add(f)
      return s;
    }),
    has: (f: File | Folder): boolean => selectedFiles.has(f),
    delete: (f: File | Folder): void => update((s) => {
      s.delete(f)
      return s;
    }),
    getOne: (): (File | Folder) | null => {
      if (selectedFiles.size === 0) {
        return null;
      }
      return Array.from(selectedFiles.values())[0];
    }
  };
}

export const selectedFiles = initSelectedFiles();
