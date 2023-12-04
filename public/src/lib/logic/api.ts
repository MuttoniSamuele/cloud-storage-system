import { account } from "../stores/account";
import { workingFolder } from "../stores/workingFolder";
import Path from "./Path";
import File from "./File"
import type { FileObject, FileType } from "./FileObject";

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

  export async function getFiles(path: Path, { foldersOnly, filter }: { foldersOnly?: boolean, filter?: FileType }): Promise<File[]> {
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
    if (!Array.isArray(json.files)) {
      throw new ApiError(`Failed to parse fetched files`);
    }
    const files: FileObject[] = json.files;
    return files.map((f) => new File(
      f.name,
      f.isFolder,
      f.fileType,
      f.owner,
      f.lastModified
    ));
  }
}

export default API;
