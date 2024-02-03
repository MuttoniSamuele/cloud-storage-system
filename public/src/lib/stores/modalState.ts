import { writable } from "svelte/store";

export enum ModalState {
  Login,
  Signup,
  TextInput,
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
