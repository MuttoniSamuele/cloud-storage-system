import { writable } from "svelte/store";

function initAccount() {
  const { subscribe, set } = writable<string | null>(null);

  return {
    subscribe,
    login: (): void => set("User"),
  };
}

export const account = initAccount();
