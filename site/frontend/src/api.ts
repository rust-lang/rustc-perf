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

export async function loadBenchmarkInfo(): Promise<BenchmarkInfo> {
  return await getJson<BenchmarkInfo>(INFO_URL);
}

export const DEFAULT_COMPILE_TARGET_TRIPLE = "x86_64-unknown-linux-gnu";
