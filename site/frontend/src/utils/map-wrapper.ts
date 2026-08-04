export class MapWrapper<T> {
  readonly data: Map<string, T>;

  constructor(data: Map<string, T>);
  constructor(data: Dict<T>);
  constructor(data: Map<string, T> | Dict<T>) {
    this.data = data instanceof Map ? data : new Map(Object.entries(data));
  }

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

  filter(predicate: (key: string, value: T) => boolean): MapWrapper<T> {
    const result = new Map<string, T>();
    for (const [key, value] of this.data) {
      if (predicate(key, value)) result.set(key, value);
    }
    return new MapWrapper(result);
  }

  reduce_entries<R>(f: (acc: R, key: string, value: T) => R, initial: R): R {
    let acc = initial;
    for (const [key, value] of this.data) {
      acc = f(acc, key, value);
    }
    return acc;
  }

  map_entries<R>(f: (key: string, value: T) => R): MapWrapper<R> {
    const result = new Map<string, R>();
    for (const [key, value] of this.data) {
      result.set(key, f(key, value));
    }
    return new MapWrapper(result);
  }

  toDict(): Dict<T> {
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
