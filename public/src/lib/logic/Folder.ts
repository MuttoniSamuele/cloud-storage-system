import type IFolder from "./IFolder";

export default class Folder implements IFolder {
  constructor(
    public readonly id: number,
    public readonly name: string,
    public readonly lastModified: string,
    public readonly starred: boolean,
    public readonly ownerId: number,
    public readonly parentId: number
  ) { }
}
