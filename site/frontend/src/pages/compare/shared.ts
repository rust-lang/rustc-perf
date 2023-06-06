export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  function padStr(i) {
    return i < 10 ? "0" + i : "" + i;
  }
  return `${date.getUTCFullYear()}-${padStr(date.getUTCMonth() + 1)}-${padStr(
    date.getUTCDate()
  )} `;
}
export function signIfPositive(pct: number): string {
  if (pct >= 0) {
    return "+";
  }
  return "";
}

export function percentClass(pct: number): string {
  let klass = "";
  if (pct > 1) {
    klass = "positive";
  } else if (pct > 0) {
    klass = "slightly-positive";
  } else if (pct < -1) {
    klass = "negative";
  } else if (pct < -0) {
    klass = "slightly-negative";
  }
  return klass;
}

export function diffClass(diff: number): string {
  let klass = "";
  if (diff > 1) {
    klass = "positive";
  } else if (diff < -1) {
    klass = "negative";
  }
  return klass;
}
