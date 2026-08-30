export function isObject(maybeObject: any): maybeObject is object {
  return (
    typeof maybeObject === "object" &&
    !Array.isArray(maybeObject) &&
    maybeObject !== null
  );
}

export function hasKey<T extends object, K extends string>(
  obj: T,
  key: K
): obj is T & Record<K, unknown> {
  return obj !== null && key in obj;
}

export function isString(maybeString: any): maybeString is string {
  return typeof maybeString === "string";
}
