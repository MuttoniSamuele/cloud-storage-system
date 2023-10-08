import { writable } from "svelte/store";

function initAccount() {
  const { subscribe, set } = writable<string | null>(null);

  return {
    subscribe,
    login: (username: string): void => set(username),
  };
}

export const account = initAccount();
