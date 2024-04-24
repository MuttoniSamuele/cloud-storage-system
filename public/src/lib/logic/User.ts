import type IUser from "./IUser";

export default class User implements IUser {
  constructor(
    public readonly username: string,
    public readonly email: string,
    public readonly personalFolderId: number,
    public readonly trashFolderId: number,
    public readonly maxStorageMb: number
  ) { }
}
