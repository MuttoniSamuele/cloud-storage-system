import { account } from "../stores/account";
import Path from "./Path";
import type { FileType } from "./fileUtils";
import type IFile from "./IFile";
import File from "./File"
import type IFolder from "./IFolder";
import Folder from "./Folder";
import { pathsHistory } from "../stores/pathsHistory";

namespace API {
  type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

  export class ApiError extends Error {
    constructor(msg: string) {
      super(msg);
      this.name = "ApiError";
    }
  }

  interface ErrorResponse {
    message: string
  }

  async function rawRequest(method: HttpMethod, url: string, headers?: Headers, jsonBody?: object): Promise<Response> {
    const res = await fetch(url, { method, headers, body: JSON.stringify(jsonBody) });
    if (res.ok) {
      // Success
      return res;
    }
    // An error happened, forward it
    let errData: ErrorResponse;
    try {
      // The server provided an error
      errData = await res.json();
    } catch {
      // Default error message
      errData = { message: "Something went wrong." }
    }
    throw new ApiError(errData.message);
  }

  export async function signup(username: string, email: string, password: string): Promise<void> {
    await rawRequest(
      "POST",
      "/api/signup",
      new Headers({ "content-type": "application/json" }),
      { username, email, password }
    );
  }

  export async function login(email: string, password: string): Promise<void> {
    // If the response is an error rawRequest raises ApiError
    await rawRequest("POST", "/api/login", new Headers({ "content-type": "application/json" }), { email, password });
    account.login("User");
    pathsHistory.push(new Path("MyCloud"));
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
