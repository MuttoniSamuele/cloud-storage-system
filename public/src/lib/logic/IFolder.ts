import type FileBase from "./FileBase";

export default interface IFolder extends FileBase {
  readonly isEmpty: boolean;
}
