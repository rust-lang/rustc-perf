import {getJson} from "./utils/requests";
import {INFO_URL} from "./urls";

export interface BenchmarkInfo {
  // Known statistic values from the DB
  stats: [string];
  // Last loaded run date
  as_of: string | null;
}

export async function loadBenchmarkInfo(): Promise<BenchmarkInfo> {
  return await getJson<BenchmarkInfo>(INFO_URL);
}
