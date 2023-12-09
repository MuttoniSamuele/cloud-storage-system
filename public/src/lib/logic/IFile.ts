import type FileBase from "./FileBase";

export type FileType = "Unsupported" | "Text" | "Image";

export default interface IFile extends FileBase {
  readonly fileType: FileType,
}
