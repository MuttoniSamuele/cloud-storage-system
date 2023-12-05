export type FileType = "Unknown" | "Text" | "Image" | "Empty" | "NotEmpty";


export interface FileObject {
  readonly name: string;
  readonly isFolder: boolean;
  readonly fileType: FileType;
  readonly owner: string;
  readonly lastModified: string;
}
