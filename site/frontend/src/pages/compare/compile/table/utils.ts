/**
 * Returns a date that is the given days in the past from the passed `date`.
 */
export function getPastDate(date: Date, days: number): Date {
  const past = new Date(date.getTime());
  past.setUTCDate(date.getUTCDate() - days);
  return past;
}

/**
 * Returns a date that is the given days in the future from the passed `date`.
 */
export function getFutureDate(date: Date, days: number): Date {
  const future = new Date(date.getTime());
  future.setUTCDate(date.getUTCDate() + days);
  return future;
}

/**
 * Returns a formated date string
 */
export function formatDate(date: Date): string {
  const year = date.getUTCFullYear();
  const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  const day = date.getUTCDate().toString().padStart(2, "0");
  return `${year}-${month}-${day}`;
}

/**
 * Calculates the number of (whole) days between the two days.
 */
export function daysBetweenDates(a: Date, b: Date): number {
  return Math.floor((b.getTime() - a.getTime()) / (1000 * 60 * 60 * 24));
}

/**
 * Provides a cache for `Output` items that can be loaded asynchronously, given a certain `Key`.
 * If a value has already been loaded for the same key, it will be resolved from the cache.
 */
export class CachedDataLoader<Key, Output> {
  private cache: Dict<Output> = {};

  constructor(
    private key_to_hash: (key: Key) => string,
    private load_fn: (key: Key) => Promise<Output>
  ) {}

  public async load(key: Key): Promise<Output> {
    const hash = this.key_to_hash(key);
    if (!this.cache.hasOwnProperty(hash)) {
      this.cache[hash] = await this.load_fn(key);
    }
    return this.cache[hash];
  }
}
