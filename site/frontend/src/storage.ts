import {ref, Ref, watch} from "vue";

/**
 * Value that is persisted in LocalStorage.
 */
class StoredValue<T> {
  private _value: T;

  constructor(private key: string, defaultValue: T) {
    this._value = getFromStorage<T>(key) ?? defaultValue;
  }

  get value(): T {
    return this._value;
  }

  store(value: T) {
    this._value = value;
    console.debug(`Changing local preference ${this.key} to ${value}`);
    setToStorage(this.key, value);
  }
}

function getFromStorage<T>(key: string): T | null {
  try {
    if (window.localStorage) {
      const found = window.localStorage.getItem(key);
      if (found !== null) {
        try {
          return JSON.parse(found) as T;
        } catch (e) {
          // Something weird is stored inside the storage key.
          // We should better remove it.
          window.localStorage.removeItem(key);
          throw e;
        }
      }
    }
  } catch (e) {
    console.error(`Error while loading \`${key}\` from local storage: ${e}`);
  }
  return null;
}

function setToStorage<T>(key: string, value: T) {
  try {
    if (window.localStorage) {
      window.localStorage.setItem(key, JSON.stringify(value));
    }
  } catch (e) {
    console.error(`Error while storing \`${key}\` to local storage: ${e}`);
  }
}

const prefix = "rustc-perf.ui";

export function createStoredValue<T>(
  key: string,
  defaultValue: T
): StoredValue<T> {
  return new StoredValue(`${prefix}.${key}`, defaultValue);
}

/**
 * Creates a reactive variable whose state will be persisted to the passed
 * `storedValue`.
 */
export function createPersistedRef<T>(storedValue: StoredValue<T>): Ref<T> {
  const value = ref(storedValue.value) as Ref<T>;
  watch(value, (newValue) => {
    storedValue.store(newValue);
  });
  return value;
}
