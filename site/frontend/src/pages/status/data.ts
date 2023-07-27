interface Commit {
  sha: string;
  date: string;
  type: "Try" | "Master";
}

interface BenchmarkStatus {
  name: string;
  error: string;
}

interface Step {
  step: string;
  is_done: boolean;
  expected_duration: number;
  current_progress: number;
}

/**
 * The `any` types in the interface below were chosen because the types are quite complex
 * on the Rust side (they are modeled with enums encoded in a way that is not so simple to model in
 * TS).
 */
interface CurrentState {
  artifact: any;
  progress: Step[];
}

export interface StatusResponse {
  last_commit: Commit | null;
  benchmarks: BenchmarkStatus[];
  missing: Array<[Commit, any]>;
  current: CurrentState | null;
  most_recent_end: number | null;
}
