import type { FileType } from "./fileUtils";
import type IFile from "./IFile";

export default class File implements IFile {
  constructor(
    public readonly name: string,
    public readonly fileType: FileType,
    public readonly owner: string,
    public readonly lastModified: string
  ) { }
}
