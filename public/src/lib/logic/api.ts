import { account } from "../stores/account";
import Path from "./Path";
import type IFile from "./IFile";
import CloudFile from "./File"
import type IFolder from "./IFolder";
import CloudFolder from "./Folder";
import { pathsHistory } from "../stores/pathsHistory";
import type IUser from "./IUser";
import User from "./User";
import { ModalState, modalState } from "../stores/modalState";
import { fileMove } from "../stores/fileMove";

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

  async function rawRequest(method: HttpMethod, url: string, headers?: Headers, body?: object | FormData, logoutOn401 = true): Promise<Response> {
    const res = await fetch(url, { method, headers, body: (body instanceof FormData ? body : JSON.stringify(body)) });
    if (logoutOn401 && res.status === 401) {
      // Unauthorized, logout
      modalState.set(ModalState.Closed);
      account.logout();
      pathsHistory.clear();
      throw new ApiError("Unauthorized");
    }
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
      { username, email, password },
      false
    );
    await loadSession();
  }

  export async function login(email: string, password: string): Promise<void> {
    // If the response is an error rawRequest raises ApiError
    await rawRequest(
      "POST",
      "/api/login",
      new Headers({ "content-type": "application/json" }),
      { email, password },
      false
    );
    await loadSession();
  }

  export async function logout(): Promise<void> {
    await rawRequest("POST", "/api/logout");
    modalState.set(ModalState.Closed);
    account.logout();
    pathsHistory.clear();
    fileMove.cancel();
  }

  export async function loadSession(): Promise<void> {
    const user = await me();
    account.login(user);
    pathsHistory.push(new Path({ id: user.personalFolderId, name: "My Cloud" }));
  }

  export async function me(): Promise<User> {
    const res = await rawRequest("GET", "/api/me", undefined, undefined, false);
    const user: IUser = await res.json();
    return new User(user.username, user.email, user.personalFolderId, user.trashFolderId, user.maxUploadMb, user.maxStorageMb);
  }

  export async function upload(file: File, parentFolderId: number): Promise<void> {
    const formData = new FormData();
    formData.append("file", file);
    formData.append("parent", parentFolderId.toString());
    await rawRequest("POST", "/api/upload", undefined, formData);
  }

  export async function view(
    parentId: number,
    foldersOnly = false
  ): Promise<{ files: CloudFile[]; folders: CloudFolder[]; }> {
    // Build the request URL
    const url = new URL("/api/view", window.location.origin);
    url.searchParams.set("parent-folder-id", parentId.toString());
    url.searchParams.set("folders-only", foldersOnly.toString());
    // Query the server
    const res = await rawRequest("GET", url.href);
    const viewResponse: { files: IFile[], folders: IFolder[] } = await res.json();
    // Turn the raw objects into instances of the corresponding classes
    return {
      files: viewResponse.files.map((f) => new CloudFile(
        f.id,
        f.name,
        f.fileType,
        f.size,
        f.lastModified,
        f.starred,
        f.ownerId,
        f.parentId
      )),
      folders: viewResponse.folders.map((f) => new CloudFolder(
        f.id,
        f.name,
        f.lastModified,
        f.starred,
        f.ownerId,
        f.parentId
      )),
    };
  }

  export async function newFolder(parentId: number, name: string): Promise<void> {
    await rawRequest("POST", "/api/folder/new", new Headers({ "content-type": "application/json" }), { parentId, name });
  }

  export async function renameFile(id: number, newName: string, isFolder = false): Promise<void> {
    await rawRequest(
      "PATCH",
      `/api/${isFolder ? "folder" : "file"}/rename`,
      new Headers({ "content-type": "application/json" }),
      { id, newName }
    );
  }

  export async function getFileContent(id: number): Promise<[string | undefined, Blob]> {
    const url = new URL("/api/file/download", window.location.origin);
    url.searchParams.set("id", id.toString());
    const res = await rawRequest("GET", url.href);
    const fileName = res.headers.get("Content-Disposition")?.split("=")[1];
    const blob = await res.blob();
    return [fileName, blob];
  }

  export async function downloadFile(id: number): Promise<void> {
    const [fileName, blob] = await getFileContent(id);
    // Create a URL for the blob
    const urlBlob = URL.createObjectURL(blob);
    // Create an anchor element to download the file
    const aElem = document.createElement("a");
    aElem.href = urlBlob;
    aElem.download = fileName ?? "default";
    aElem.style.display = "none";
    // Add the element to the DOM, click it, and remove it
    document.body.appendChild(aElem);
    aElem.click();
    document.body.removeChild(aElem);
    URL.revokeObjectURL(urlBlob);
  }

  export async function getFolderSize(id: number, filter: string | null = null): Promise<number> {
    const url = new URL("/api/folder/size", window.location.origin);
    url.searchParams.set("id", id.toString());
    if (filter !== null) {
      url.searchParams.set("filter", filter);
    }
    const res = await rawRequest("GET", url.href);
    const { size }: { size: number } = await res.json();
    return size;
  }

  export async function moveFile(id: number, folderId: number, isFolder = false): Promise<void> {
    await rawRequest(
      "PATCH",
      `/api/${isFolder ? "folder" : "file"}/move`,
      new Headers({ "content-type": "application/json" }),
      { id, folderId }
    );
  }

  export async function deleteFile(id: number, isFolder = false, preserveParent = false): Promise<void> {
    const url = new URL(`/api/${isFolder ? "folder" : "file"}/delete`, window.location.origin);
    url.searchParams.set("id", id.toString());
    url.searchParams.set("preserve-parent", preserveParent.toString());
    await rawRequest("DELETE", url.href);
  }

  export async function deleteMe(): Promise<void> {
    await rawRequest("DELETE", "/api/me");
    await logout();
  }
}

export default API;
