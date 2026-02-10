import {getJson} from "./utils/requests";
import {INFO_URL} from "./urls";

export interface BenchmarkInfo {
  // Known compile metrics from the DB
  compile_metrics: string[];
  // Known runtime metrics from the DB
  runtime_metrics: string[];
  // Known targets that have been benchmarked from the DB
  compile_targets: string[];
}

export const DEFAULT_COMPILE_TARGET_TRIPLE = "x86_64-unknown-linux-gnu";

function compareDefaultCompileTarget(a: string, b: string) {
  return (
    a === DEFAULT_COMPILE_TARGET_TRIPLE && b !== DEFAULT_COMPILE_TARGET_TRIPLE
  );
}

export async function loadBenchmarkInfo(): Promise<BenchmarkInfo> {
  const benchmarkInfo = await getJson<BenchmarkInfo>(INFO_URL);
  benchmarkInfo.compile_targets.sort((a, b) => {
    // Ensure the default target always appears first, then do an alphabetical
    // sort
    if (compareDefaultCompileTarget(a, b)) return -1;
    if (compareDefaultCompileTarget(b, a)) return 1;
    return a.localeCompare(b);
  });
  return benchmarkInfo;
}
