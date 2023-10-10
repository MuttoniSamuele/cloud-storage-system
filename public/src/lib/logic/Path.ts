export default class Path {
  private path: string[];

  constructor();
  constructor(path: string[]);
  constructor(baseFolder: string);
  constructor(param?: string[] | string) {
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

  public get rawPath(): string[] {
    return [...this.path];
  }

  public addSubFolder(folder: string): void {
    this.path.push(folder);
  }

  public contains(subPath: Path): boolean {
    if (subPath.path.length > this.path.length) {
      return false;
    }
    for (let i = 0; i < subPath.path.length; i++) {
      if (subPath.path[i] !== this.path[i]) {
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
      if (this.path[i] !== other.path[i]) {
        return false;
      }
    }
    return true;
  }

  public clone(): Path {
    return new Path(this.path);
  }

  public toString(): string {
    return "/" + this.path.join("/");
  }
}
