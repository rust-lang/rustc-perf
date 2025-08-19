function getTypeString(obj: any): string {
  return Object.prototype.toString
    .call(obj)
    .toLowerCase()
    .replace("[", "")
    .replace("]", "")
    .split(" ")[1];
}

export function isObject(maybeObject: any): maybeObject is object {
  return getTypeString(maybeObject) === "object";
}

export function isString(maybeString: any): maybeString is string {
  return getTypeString(maybeString) === "string";
}
