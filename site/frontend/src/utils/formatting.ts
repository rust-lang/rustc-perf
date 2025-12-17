import {parseISO, format} from "date-fns";

// `time` has to be in seconds
export function formatSecondsAsDuration(time: number): string {
  let seconds = time % 60;
  let mins = Math.trunc(time / 60);
  let hours = Math.trunc(mins / 60);
  mins -= hours * 60;

  let s = "";
  if (hours > 0) {
    s = `${hours}h ${mins < 10 ? "0" + mins : mins}m ${
      seconds < 10 ? "0" + seconds : seconds
    }s`;
  } else {
    s = `${mins < 10 ? " " + mins : mins}m ${
      seconds < 10 ? "0" + seconds : seconds
    }s`;
  }
  return s;
}

export const DATE_FMT_KEY = "__rustc-perf-user-date-fmt-preference__";
// Date formats taken from https://date-fns.org/v4.1.0/docs/format
export const DATE_FMT_24HR = "yyyy-MM-dd HH:mm:ss";
export const DATE_FMT_12HR = "yyyy-MM-dd hh:mm:ss a";

export function setDateFmt(dateFmt: string) {
  window.localStorage.setItem(DATE_FMT_KEY, dateFmt);
}

export function getDateFmt() {
  return window.localStorage.getItem(DATE_FMT_KEY) ?? DATE_FMT_24HR;
}

// Takes a date like `2025-09-10T08:22:47.161348Z` and formats it according to
// the user preference stored in local storage (either 12 hour or 24 hour format).
export function formatISODate(dateString?: string): string {
  if (dateString) {
    const dateFmt = getDateFmt();
    return format(parseISO(dateString), dateFmt);
  }
  return "";
}

export function parseDateIsoStringOrNull(dateString?: string): Date | null {
  if (dateString) {
    try {
      return parseISO(dateString);
    } catch (e) {
      return null;
    }
  }
  return null;
}
