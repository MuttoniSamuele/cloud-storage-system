import { account } from "../stores/account";
import Path from "./Path";
import type { FileType } from "./fileUtils";
import type IFile from "./IFile";
import CloudFile from "./File"
import type IFolder from "./IFolder";
import CloudFolder from "./Folder";
import { pathsHistory } from "../stores/pathsHistory";
import type IUser from "./IUser";
import User from "./User";

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

  // TODO: Make it logout if the server responds with 401
  async function rawRequest(method: HttpMethod, url: string, headers?: Headers, body?: object | FormData): Promise<Response> {
    const res = await fetch(url, { method, headers, body: (body instanceof FormData ? body : JSON.stringify(body)) });
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
    await loadSession();
  }

  export async function login(email: string, password: string): Promise<void> {
    // If the response is an error rawRequest raises ApiError
    await rawRequest("POST", "/api/login", new Headers({ "content-type": "application/json" }), { email, password });
    await loadSession();
  }

  export async function logout(): Promise<void> {
    await rawRequest("POST", "/api/logout");
    account.logout();
    pathsHistory.clear();
  }

  export async function loadSession(): Promise<void> {
    const user = await me();
    account.login(user);
    pathsHistory.push(new Path("MyCloud"));
  }

  export async function me(): Promise<User> {
    let res = await rawRequest("GET", "/api/me");
    const user: IUser = await res.json();
    return new User(user.username, user.email);
  }

  export async function upload(file: File) {
    const formData = new FormData();
    formData.append("file", file);
    await rawRequest("POST", "/api/upload", undefined, formData);
  }

  export async function getFiles(
    path: Path,
    { foldersOnly, filter }: { foldersOnly?: boolean, filter?: FileType } = {}
  ): Promise<{ files: CloudFile[]; folders: CloudFolder[] }> {
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
      files: files.map((f) => new CloudFile(
        f.name,
        f.fileType,
        f.owner,
        f.lastModified
      )),
      folders: folders.map((f) => new CloudFolder(
        f.name,
        f.isEmpty,
        f.owner,
        f.lastModified
      )),
    };
  }
}

export default API;
