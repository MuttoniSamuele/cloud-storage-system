import { writable } from "svelte/store";
import type Path from "../logic/Path";

type PathHistory = { paths: (Path | null)[], index: number };

function initPathsHistory() {
  const { subscribe, update } = writable<PathHistory>({ paths: [null], index: 0 });

  return {
    subscribe,
    push: (path: Path): void => {
      update(({ paths, index }) => {
        return {
          paths: [...paths.slice(0, index + 1), path.clone()],
          index: index + 1
        };
      })
    },
    goBack: (): void => {
      update((history: PathHistory) => {
        history.index = Math.max(0, history.index - 1);
        return history;
      });
    },
    goForward: (): void => {
      update((history: PathHistory) => {
        history.index = Math.min(history.index + 1, history.paths.length - 1);
        return history;
      });
    },
    refresh: (): void => {
      update((history: PathHistory) => history);
    }
  };
}

export function getCurrentPath({ paths, index }: PathHistory) {
  return paths[index]?.clone() || null;
}

export const pathsHistory = initPathsHistory();
