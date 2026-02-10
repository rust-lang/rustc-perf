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

export async function loadBenchmarkInfo(): Promise<BenchmarkInfo> {
  const benchmarkInfo = await getJson<BenchmarkInfo>(INFO_URL);
  benchmarkInfo.compile_targets.sort((a, b) => {
    // Ensure the default target always appears first, then do an alphabetical
    // sort
    if (a === DEFAULT_COMPILE_TARGET_TRIPLE) {
      return 2;
    } else if (a < b) {
      return 1;
    } else if (a > b) {
      return -1;
    } else return 0;
  });
  return benchmarkInfo;
}
