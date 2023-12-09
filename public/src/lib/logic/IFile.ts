import type FileBase from "./FileBase";
import type { FileType } from "./fileUtils";

export default interface IFile extends FileBase {
  readonly fileType: FileType,
}
