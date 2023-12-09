import type FileBase from "./FileBase";

export type FileType = "Unsupported" | "Text" | "Image";

export function cmpFileNames(f1: FileBase, f2: FileBase) {
  return f1.name.localeCompare(f2.name);
}
