import type IUser from "./IUser";

export default class User implements IUser {
  constructor(
    public readonly username: string,
    public readonly email: string
  ) { }
}