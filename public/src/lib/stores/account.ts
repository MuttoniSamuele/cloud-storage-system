import { writable } from "svelte/store";
import type User from "../logic/User";

function initAccount() {
  const { subscribe, set } = writable<User | null>(null);

  return {
    subscribe,
    login: (username: User): void => set(username),
    logout: (): void => set(null)
  };
}

export const account = initAccount();
