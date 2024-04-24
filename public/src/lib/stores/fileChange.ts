import { writable } from "svelte/store";

export interface FileChange {
  file: string | null;
  folder: string | null;
}

function initFileChange() {
  const { subscribe, update } = writable<FileChange>({ file: null, folder: null });

  return {
    subscribe,
    setFile: (fileName: string) => update((state) => ({ ...state, file: fileName })),
    setFolder: (folderName: string) => update((state) => ({ ...state, folder: folderName }))
  };
}

export const fileChange = initFileChange();
