import { account } from "../stores/account";
import { workingFolder } from "../stores/workingFolder";
import Path from "./Path";

namespace API {
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

  export async function fetchSubfolders(path: Path): Promise<
    { name: string; empty: boolean }[]
  > {
    const res = await fetch(`/dummy/folders?path=${path.toString()}`);
    const json = await res.json();
    return json.folders;
  }
}

export default API;
