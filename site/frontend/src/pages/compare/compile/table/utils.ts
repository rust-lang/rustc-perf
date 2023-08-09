/**
 * Returns a date that is the given days in the past from the passed `date`.
 */
export function getPastDate(date: Date, days: number): string {
  const past = new Date(date.getTime());
  past.setUTCDate(date.getUTCDate() - days);
  return format_ymd(past);
}

/**
 * Returns a date that is the given days in the future from the passed `date`.
 */
export function getFutureDate(date: Date, days: number): string {
  const future = new Date(date.getTime());
  future.setUTCDate(date.getUTCDate() + days);
  return format_ymd(future);
}

function format_ymd(date: Date): string {
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
