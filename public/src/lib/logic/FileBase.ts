export default interface FileBase {
  readonly id: number,
  readonly name: string;
  readonly lastModified: string;
  readonly starred: boolean,
  readonly ownerId: number;
  readonly parentId: number;
}
