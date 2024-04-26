export default interface IUser {
  readonly username: string,
  readonly email: string,
  readonly personalFolderId: number,
  readonly trashFolderId: number,
  readonly maxUploadMb: number,
  readonly maxStorageMb: number,
}
