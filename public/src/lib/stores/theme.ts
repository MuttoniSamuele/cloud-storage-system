import { writable } from "svelte/store";

type Theme = "light" | "dark";
// The key of the item in local storage
const THEME_KEY = "theme";

// Reads from local storage or, if not present, from the system preferences
function readTheme(): Theme {
  let savedTheme = localStorage.getItem(THEME_KEY);
  if (
    savedTheme === "dark" ||
    (savedTheme === null &&
      window.matchMedia("(prefers-color-scheme: dark)").matches)
  ) {
    return "dark";
  }
  return "light";
}

// Applies the theme to the page and saves it to local storage
function applyTheme(theme: Theme): void {
  if (theme === "dark") {
    document.documentElement.classList.add("dark");
    localStorage.setItem(THEME_KEY, "dark");
  } else {
    document.documentElement.classList.remove("dark");
    localStorage.setItem(THEME_KEY, "light");
  }
}

// The theme store
function initTheme() {
  const { subscribe, update } = writable<Theme>(readTheme());

  return {
    subscribe,
    toggle: (): void => {
      update((t) => t === "dark" ? "light" : "dark");
    }
  };
}

export const theme = initTheme();
// Apply the theme to the page everytime the store updates
theme.subscribe((t) => applyTheme(t));
