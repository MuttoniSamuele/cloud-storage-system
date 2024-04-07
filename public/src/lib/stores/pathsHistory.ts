import { writable } from "svelte/store";
import type Path from "../logic/Path";

type PathHistory = { paths: (Path | null)[], index: number };

function initPathsHistory() {
  // The history of paths is stored as a combination of an array of Path and
  // the index of the current path in said array.
  const { subscribe, update, set } = writable<PathHistory>({ paths: [null], index: 0 });

  return {
    subscribe,
    push: (path: Path): void => {
      update(({ paths, index }) => {
        const prevPath = getCurrentPath({ paths, index });
        // Don't store the path if it's the same as the previous one
        if (prevPath !== null && prevPath.cmp(path)) {
          return {
            paths: paths.slice(0, index + 1),
            index
          };
        }
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
    },
    clear: (): void => {
      set({ paths: [null], index: 0 })
    }
  };
}

export function getCurrentPath({ paths, index }: PathHistory): Path | null {
  return paths[index]?.clone() || null;
}

export const pathsHistory = initPathsHistory();
