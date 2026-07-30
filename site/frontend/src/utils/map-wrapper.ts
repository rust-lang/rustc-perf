export class MapWrapperE<T> {
  constructor(public readonly data: Map<string, T>) {}

  get(key: string): T | undefined {
    return this.data.get(key);
  }
  has(key: string): boolean {
    return this.data.has(key);
  }
  keys(): MapIterator<string> {
    return this.data.keys();
  }
  values(): MapIterator<T> {
    return this.data.values();
  }
  entries(): MapIterator<[string, T]> {
    return this.data.entries();
  }
  get size(): number {
    return this.data.size;
  }
  [Symbol.iterator](): MapIterator<[string, T]> {
    return this.data[Symbol.iterator]();
  }

  filter(predicate: (key: string, value: T) => boolean): Map<string, T> {
    const result = new Map<string, T>();
    for (const [key, value] of this.data) {
      if (predicate(key, value)) result.set(key, value);
    }
    return result;
  }

  toJSON(): Dict<T> {
    return Object.fromEntries(this.data.entries());
  }
}

/**
 * Helper function for creating Maps from json presentation when we know how to create children
 * from json presentation
 */
export function mapFromJSON<S, T>(
  json: Dict<S>,
  valueFromJSON: (value: S) => T
): Map<string, T> {
  return new Map(Object.entries(json).map(([k, v]) => [k, valueFromJSON(v)]));
}
