import type { FileType } from "./fileUtils";
import type IFile from "./IFile";

export default class File implements IFile {
  public readonly name: string;
  public readonly extension: string | null;

  constructor(
    name: string,
    public readonly fileType: FileType,
    public readonly owner: string,
    public readonly lastModified: number
  ) {
    const fileNameArr = name.split(".");
    // Check if there was at least a "."
    if (fileNameArr.length >= 2) {
      // Use the last element as the extension
      this.extension = fileNameArr.pop() as string;
      // Join the rest of the name back with "."
      this.name = fileNameArr.join(".");
    } else {
      // Otherwise it only has a name
      this.name = fileNameArr[0];
      // And no extension
      this.extension = null;
    }
  }
}
