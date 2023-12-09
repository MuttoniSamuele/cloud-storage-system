import type IFolder from "./IFolder";

export default class Folder implements IFolder {
  constructor(
    public readonly name: string,
    public readonly isEmpty: boolean,
    public readonly owner: string,
    public readonly lastModified: number
  ) { }
}
