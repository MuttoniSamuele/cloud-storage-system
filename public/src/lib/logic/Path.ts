export interface PathFolder {
  id: number;
  name: string;
}

export default class Path {
  private path: PathFolder[];

  constructor();
  constructor(rawPath: PathFolder[]);
  constructor(baseFolder: PathFolder);
  constructor(param?: PathFolder[] | PathFolder) {
    if (param === undefined) {
      this.path = [];
      return;
    }
    if (Array.isArray(param)) {
      this.path = [...param];
    } else {
      this.path = [param];
    }
  }

  public get rawPath(): PathFolder[] {
    return [...this.path];
  }

  public get lastId(): number {
    return this.rawPath.reverse()[0].id;
  }

  public addSubFolder(folder: PathFolder): void {
    this.path.push(folder);
  }

  public contains(subPath: Path): boolean {
    if (subPath.path.length > this.path.length) {
      return false;
    }
    for (let i = 0; i < subPath.path.length; i++) {
      if (subPath.path[i].id !== this.path[i].id) {
        return false;
      }
    }
    return true;
  }

  public cmp(other: Path): boolean {
    if (this.path.length !== other.path.length) {
      return false;
    }
    for (let i = 0; i < this.path.length; i++) {
      if (this.path[i].id !== other.path[i].id) {
        return false;
      }
    }
    return true;
  }

  public clone(): Path {
    return new Path(this.path);
  }

  public toString(): string {
    return "/" + this.path.map(({ name }) => name).join("/");
  }
}
