import type FileBase from "./FileBase";

export function cmpFileNames(f1: FileBase, f2: FileBase): number {
  return f1.name.localeCompare(f2.name, undefined, { numeric: true });
}

export function fileTypeToIcon(fileType: string | null): string {
  switch (fileType) {
    case "Image": { return "ri-image-2-fill" }
    case "Text": { return "ri-file-text-fill" }
    default: { return "ri-file-2-fill" }
  }
}
