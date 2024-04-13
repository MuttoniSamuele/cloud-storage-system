export default interface FileBase {
  readonly id: number,
  readonly name: string;
  readonly owner: string;
  readonly lastModified: number;
}
