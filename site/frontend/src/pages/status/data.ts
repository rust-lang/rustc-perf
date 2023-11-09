export interface Commit {
  sha: string;
  date: string;
  type: "Try" | "Master";
}

export interface BenchmarkError {
  name: string;
  error: string;
}

interface Step {
  step: string;
  is_done: boolean;
  expected_duration: number;
  current_progress: number;
}

export type Artifact =
  | {
      Commit: Commit;
    }
  | {
      Tag: string;
    };

export type MissingReason =
  | {
      Master: {
        pr: number;
        parent_sha: string;
        is_try_parent: boolean;
      };
    }
  | {
      Try: {
        pr: number;
        parent_sha: string;
        include: string | null;
        exclude: string | null;
        runs: number | null;
      };
    }
  | {
      InProgress: MissingReason;
    };

interface CurrentState {
  artifact: Artifact;
  progress: Step[];
}

export interface FinishedRun {
  artifact: Artifact;
  pr: number | null;
  errors: BenchmarkError[];
  duration: number;
  finished_at: number;
}

export interface StatusResponse {
  finished_runs: FinishedRun[];
  current: CurrentState | null;
  missing: Array<[Commit, MissingReason]>;
}
