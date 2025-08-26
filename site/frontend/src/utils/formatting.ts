export function formatDuration(milliseconds: number): string {
  let seconds = milliseconds / 1000;
  let secs = seconds % 60;
  let mins = Math.trunc(seconds / 60);
  let hours = Math.trunc(mins / 60);
  mins -= hours * 60;

  let s = "";
  if (hours > 0) {
    s = `${hours}h ${mins < 10 ? "0" + mins : mins}m ${
      secs < 10 ? "0" + secs : secs
    }s`;
  } else {
    s = `${mins < 10 ? " " + mins : mins}m ${secs < 10 ? "0" + secs : secs}s`;
  }
  return s;
}
