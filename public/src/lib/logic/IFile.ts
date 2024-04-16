import type FileBase from "./FileBase";

export default interface IFile extends FileBase {
  readonly fileType: string | null;
  readonly size: number;
}
