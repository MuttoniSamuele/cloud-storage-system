import { writable } from "svelte/store";

export enum ModalState {
  Login,
  Signup,
  Upload,
  NewFolder,
  Rename,
  TextFile,
  ImageFile,
  UnsupportedFile,
  Closed
}

function initModalState() {
  const { subscribe, set } = writable<ModalState>(ModalState.Closed);

  return {
    subscribe,
    set: (s: ModalState): void => set(s),
  };
}

export const modalState = initModalState();
