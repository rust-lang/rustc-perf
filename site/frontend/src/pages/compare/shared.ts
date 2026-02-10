import {Target} from "./compile/common";

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

export function getStringArrayOrDefault(
  params: Dict<string>,
  name: string,
  defaultValue: string[]
): string[] {
  if (params.hasOwnProperty(name)) {
    return params[name].split(",");
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

export function targetMatchesFilter(
  target: Target,
  target_set: Target[]
): boolean {
  return target_set.includes(target);
}

// Store the bool value into URL parameters, or reset it if it has the default
// value.
export function storeOrResetBool<T extends boolean | string>(
  urlParams: Dict<string>,
  name: string,
  value: T,
  defaultValue: T
) {
  if (value === defaultValue) {
    if (urlParams.hasOwnProperty(name)) {
      delete urlParams[name];
    }
  } else {
    urlParams[name] = value.toString();
  }
}

export function storeOrResetStringArray(
  urlParams: Dict<string>,
  name: string,
  value: string[],
  defaultValue: string[]
) {
  if (isSameStringArray(value, defaultValue)) {
    if (urlParams.hasOwnProperty(name)) {
      delete urlParams[name];
    }
  } else {
    urlParams[name] = value.sort().join(",");
  }
}

export function loadTargetsFromUrl(
  urlParams: Dict<string>,
  defaultTargets: Target[]
): Target[] {
  return getStringArrayOrDefault(
    urlParams,
    "target",
    defaultTargets
  ) as Target[];
}

// I hate JavaScript...
export function isSameStringArray(a: string[], b: string[]): boolean {
  return a.length === b.length && a.sort().join(",") === b.sort().join(",");
}

const TARGET_SHORTCUTS: {[target in Target]: string} = {
  "x86_64-unknown-linux-gnu": "x64",
  "aarch64-unknown-linux-gnu": "AArch64",
};

export function formatTarget(target: Target): string {
  return TARGET_SHORTCUTS[target] ?? target?.split("-")[0];
}
