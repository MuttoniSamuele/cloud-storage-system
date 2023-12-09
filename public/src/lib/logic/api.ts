import { account } from "../stores/account";
import { workingFolder } from "../stores/workingFolder";
import Path from "./Path";
import type { FileType } from "./IFile";
import type IFile from "./IFile";
import File from "./File"
import type IFolder from "./IFolder";
import Folder from "./Folder";

namespace API {
  export class ApiError extends Error {
    constructor(msg: string) {
      super(msg);
      this.name = "ApiError";
    }
  }

  export async function login(username: string, password: string): Promise<void> {
    const options = {
      method: "POST",
      headers: new Headers({ "content-type": "application/json" }),
      body: JSON.stringify({ username: username, password: password }),
    };
    const res = await fetch("/dummy/login", options);
    if (res.ok) {
      account.login(username);
      workingFolder.change(new Path("MyCloud"));
    }
  }

  export async function getFiles(
    path: Path,
    { foldersOnly, filter }: { foldersOnly?: boolean, filter?: FileType } = {}
  ): Promise<{ files: File[]; folders: Folder[] }> {
    const url = new URL("/dummy/files", window.location.origin);
    url.searchParams.set("path", path.toString());
    url.searchParams.set("folders-only", (foldersOnly || false).toString());
    if (filter !== undefined) {
      url.searchParams.set("types", filter);
    }
    const res = await fetch(url.href);
    if (!res.ok) {
      throw new ApiError(`Failed to fetch the files: code ${res.status}`);
    }
    const json = await res.json();
    // TODO: Add controls for object properties
    if (!Array.isArray(json.files) || !Array.isArray(json.folders)) {
      throw new ApiError(`Failed to parse fetched files`);
    }
    const files: IFile[] = json.files;
    const folders: IFolder[] = json.folders;
    return {
      files: files.map((f) => new File(
        f.name,
        f.fileType,
        f.owner,
        f.lastModified
      )),
      folders: folders.map((f) => new Folder(
        f.name,
        f.isEmpty,
        f.owner,
        f.lastModified
      )),
    };
  }
}

export default API;
