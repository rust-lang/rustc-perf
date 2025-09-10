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

// Takes a date like `2025-09-10T08:22:47.161348Z` -> `"2025-09-10 08:22:47"`
export function formatISODate(dateString?: string): string {
  if (dateString) {
    return format(parseISO(dateString), "yyyy-MM-dd HH:mm:ss");
  }
  return "";
}
