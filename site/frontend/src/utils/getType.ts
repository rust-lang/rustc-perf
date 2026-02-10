export function isObject(maybeObject: any): maybeObject is object {
  return (
    typeof maybeObject === "object" &&
    !Array.isArray(maybeObject) &&
    maybeObject !== null
  );
}

export function isString(maybeString: any): maybeString is string {
  return typeof maybeString === "string";
}
