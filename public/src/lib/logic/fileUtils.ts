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

export function formatBytes(bytes: number, precision = 2): string {
  const units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  let i = 0;
  while (bytes >= 1000 && i < units.length - 1) {
    bytes /= 1000;
    i++;
  }
  return parseFloat(bytes.toFixed(precision)).toString() + " " + units[i];
}

// TODO: Fix wrong timezone
export function formatLastModified(lastModified: string): string {
  return new Intl.DateTimeFormat(navigator.language, {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
  }).format(new Date(lastModified));
}

