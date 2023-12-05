import type { FileObject, FileType } from "./FileObject";

export default class File implements FileObject {
  constructor(
    public readonly name: string,
    public readonly isFolder: boolean,
    public readonly fileType: FileType,
    public readonly owner: string,
    public readonly lastModified: string
  ) { }

  get isFile() {
    return !this.isFolder;
  }
}
