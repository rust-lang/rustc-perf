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
  if (diff >= 0) {
    return "positive";
  }
  return "negative";
}

const KiB = 1024;
const MiB = KiB * 1024;
const GiB = MiB * 1024;

export function formatSize(size: number): string {
  if (size >= GiB) {
    return `${(size / GiB).toFixed(2)} GiB`;
  } else if (size >= MiB) {
    return `${(size / MiB).toFixed(2)} MiB`;
  } else if (size >= KiB) {
    return `${(size / KiB).toFixed(2)} KiB`;
  }
  return `${size} B`;
}

// Formats a percentual change between two values
export function formatPercentChange(
  before: number | undefined,
  after: number | undefined,
  invalidPlaceholder: string = "-"
): string {
  if (!isValidValue(before) || !isValidValue(after)) {
    return invalidPlaceholder;
  }
  return `${(((after - before) / before) * 100).toFixed(3)}%`;
}

// Checks if the value is not undefined and not zero
export function isValidValue(size: number | undefined): boolean {
  return size !== undefined && size !== 0;
}

export function getBoolOrDefault(
  params: Dict<string>,
  name: string,
  defaultValue: boolean
): boolean {
  if (params.hasOwnProperty(name)) {
    return params[name] === "true";
  }
  return defaultValue;
}

export function benchmarkNameMatchesFilter(
  benchmarkName: string,
  filterName: string | null
): boolean {
  if (!filterName) return true;
  const trimmedFilterName = filterName.trim();
  // Use `includes()` when a regex is not valid.
  try {
    const filterRegex = new RegExp(trimmedFilterName);
    return filterRegex.test(benchmarkName);
  } catch (e) {
    return benchmarkName.includes(trimmedFilterName);
  }
}
