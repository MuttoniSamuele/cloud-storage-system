import type FileBase from "./FileBase";

export type FileType = "Unsupported" | "Text" | "Image";

export function cmpFileNames(f1: FileBase, f2: FileBase): number {
  return f1.name.localeCompare(f2.name);
}

export function fileTypeToIcon(fileType: FileType): string {
  switch (fileType) {
    case "Image": { return "ri-image-2-fill" }
    case "Text": { return "ri-file-text-fill" }
    default: { return "ri-file-2-fill" }
  }
}
