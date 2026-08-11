import {hasKey, isObject} from "./getType.ts";

export class MapWrapper<K extends string, V> {
  private readonly data: Map<K, V>;

  constructor(data: Map<K, V>);
  constructor(data: Dict<V>);
  constructor(data: Map<K, V> | Dict<V>) {
    this.data =
      data instanceof Map ? data : new Map(Object.entries(data) as [K, V][]);
  }

  get(key: K): V {
    const result: V | undefined = this.data.get(key);
    if (result === undefined) {
      console.debug(
        `unknown key: '${key}'. Existing keys:`,
        Array.from(this.keys())
      );
      throw new EvalError(`unknown key: '${key}'`);
    }
    return result;
  }
  set(key: K, value): this {
    this.data.set(key, value);
    return this;
  }
  delete(key: K): boolean {
    return this.data.delete(key);
  }

  has(key: K): boolean {
    return this.data.has(key);
  }
  keys(): MapIterator<K> {
    return this.data.keys();
  }
  values(): MapIterator<V> {
    return this.data.values();
  }
  entries(): MapIterator<[K, V]> {
    return this.data.entries();
  }
  get size(): number {
    return this.data.size;
  }
  [Symbol.iterator](): MapIterator<[K, V]> {
    return this.data[Symbol.iterator]();
  }

  filter(predicate: (key: K, value: V) => boolean): this {
    const result = new Map<K, V>();
    for (const [key, value] of this.data) {
      if (predicate(key, value)) result.set(key, value);
    }
    return new (this.constructor as new (data: Map<K, V>) => this)(result);
  }

  reduce_entries<V2>(f: (acc: V2, key: K, value: V) => V2, initial: V2): V2 {
    let acc = initial;
    for (const [key, value] of this.data) {
      acc = f(acc, key, value);
    }
    return acc;
  }

  map_entries<V2>(f: (key: K, value: V) => V2): MapWrapper<K, V2> {
    const result = new Map<K, V2>();
    for (const [key, value] of this.data) {
      result.set(key, f(key, value));
    }
    return new MapWrapper(result);
  }

  toJSON(): Record<K, V | string> {
    let obj = Object.create(null) as Record<K, V | string>;
    for (let [k, v] of this.data.entries()) {
      if (v && isObject(v)) {
        if (
          hasKey(v, "toJSON") &&
          typeof v.toJSON == "function" &&
          v.toJSON.length == 0
        ) {
          obj[k] = v.toJSON();
        } else {
          obj[k] = v;
        }
      } else {
        obj[k] = JSON.stringify(v);
      }
    }
    return obj;
  }
}

/**
 * Helper function for creating Maps from json presentation when we know how to create children
 * from json presentation
 */
export function mapFromJSON<K extends string, S, T>(
  json: Record<K, S>,
  keyConverter: (key: string) => K,
  valueConverter: (value: S) => T
): Map<K, T> {
  return new Map(
    Object.entries(json).map(([k, v]) => [
      keyConverter(k),
      valueConverter(v as S),
    ])
  );
}
