import {getJson} from "./utils/requests";
import {INFO_URL} from "./urls";

export interface BenchmarkInfo {
  // Known compile metrics from the DB
  compile_metrics: [string];
  // Known runtime metrics from the DB
  runtime_metrics: [string];
  // Last loaded run date
  as_of: string | null;
}

export async function loadBenchmarkInfo(): Promise<BenchmarkInfo> {
  return await getJson<BenchmarkInfo>(INFO_URL);
}
