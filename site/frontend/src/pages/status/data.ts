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

type Artifact =
  | {
      Commit: Commit;
    }
  | {
      Tag: string;
    };

type MissingReason =
  | {
      Master: {
        pr: number;
        parent_sha: string;
        is_try_parent: boolean;
      };
    }
  | {
      Try: {
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

export interface StatusResponse {
  last_commit: Commit | null;
  benchmarks: BenchmarkStatus[];
  missing: Array<[Commit, MissingReason]>;
  current: CurrentState | null;
  most_recent_end: number | null;
}
