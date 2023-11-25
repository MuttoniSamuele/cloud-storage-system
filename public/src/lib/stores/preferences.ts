import { writable } from "svelte/store";

const PREFERENCES_KEY = "preferences";

interface Preferences {
  filesLayout: "grid" | "rows"
}

// Returns a default preferences object
function createDefaultPreferences(): Preferences {
  return {
    filesLayout: "grid"
  };
}

// Loads preferences from localstorage. If no preferences are found, returns
// the default preferences
function loadPreferences(): Preferences {
  const data = localStorage.getItem(PREFERENCES_KEY);
  // Check if there are no saved preferences
  if (data === null) {
    return createDefaultPreferences();
  }
  // Try parsing the preferences
  try {
    return JSON.parse(data)
  } catch {
    return createDefaultPreferences();
  }
}

// Saves preferences to localstorage
function savePreferences(preferences: Preferences): void {
  localStorage.setItem(PREFERENCES_KEY, JSON.stringify(preferences));
}

// The preferences store
function initPreferences() {
  const { subscribe, update } = writable<Preferences>(loadPreferences());

  return {
    subscribe,
    toggleFilesLayout: (): void => {
      update((p) => {
        p.filesLayout = (p.filesLayout == "grid" ? "rows" : "grid");
        return p;
      });
    }
  };
}

export const preferences = initPreferences();
// Save the preferences everytime the store updates
preferences.subscribe((p) => savePreferences(p));
