import {getTypeString} from "../../utils/getType";

type CommitTypeMaster = {
  sha: string;
  parent_sha: string;
  pr: number;
};

type CommitTypeRelease = {
  tag: string;
};

type CommitTypeTry = {
  sha: string;
  parent_sha: string | null;
  pr: number;
};

export type CommitType = CommitTypeRelease | CommitTypeMaster | CommitTypeTry;
export type CommitTypeString = "Master" | "Try" | "Release";

export type BenchmarkRequestComplete = {
  commit_type: {
    [K in CommitTypeString]: CommitType;
  };
  commit_date: string | null;
  created_at: string | null;
  status: {
    Completed: {
      completed_at: string;
      duration_ms: number;
    };
  };
  backends: string;
  profile: string;
};

export type BenchmarkRequestInProgress = {
  commit_type: {
    [K in CommitTypeString]: CommitType;
  };
  commit_date: string | null;
  created_at: string | null;
  status: "InProgress";
  backends: string;
  profiles: string;
};

export function isMasterBenchmarkRequest(
  commitType: Object
): commitType is {["Master"]: CommitTypeMaster} {
  return "Master" in commitType;
}

export function isReleaseBenchmarkRequest(
  commitType: Object
): commitType is {["Release"]: CommitTypeRelease} {
  return "Release" in commitType;
}

export function isTryBenchmarkRequest(
  commitType: Object
): commitType is {["Try"]: CommitTypeTry} {
  return "Try" in commitType;
}

export type BenchmarkJobStatusInProgress = {
  started_at: string;
  collector_name: string;
};

export type BenchmarkJobStatusCompleted = {
  started_at: string;
  completed_at: string;
  collector_name: string;
  success: boolean;
};

export type BenchmarkJobStatusString = "InProgress" | "Completed";
export type BenchmarkJobStatusQueued = "Queued";

export type BenchmarkJob = {
  id: number;
  target: string;
  backend: string;
  request_tag: string;
  benchmark_set: number;
  created_at: string;
  status:
    | BenchmarkJobStatusQueued
    | {
        [K in BenchmarkJobStatusQueued]:
          | BenchmarkJobStatusInProgress
          | BenchmarkJobStatusCompleted;
      };
  deque_counter: number;
};

export function isQueuedBenchmarkJob(
  status: unknown
): status is BenchmarkJobStatusQueued {
  return getTypeString(status) === "string";
}

export function isInProgressBenchmarkJob(
  status: unknown
): status is {["InProgress"]: BenchmarkJobStatusInProgress} {
  return getTypeString(status) === "object" && "InProgress" in status;
}

export function isCompletedBenchmarkJob(
  status: unknown
): status is {["Completed"]: BenchmarkJobStatusCompleted} {
  return getTypeString(status) === "object" && "Completed" in status;
}

export type CollectorConfig = {
  name: string;
  target: string;
  benchmark_set: number;
  is_active: boolean;
  last_heartbeat_at: string;
  date_added: string;
};

export type StatusResponse = {
  completed: [BenchmarkRequestComplete, string[]][];
  in_progress: [BenchmarkRequestInProgress, BenchmarkJob[]][];
  collector_configs: CollectorConfig[];
};
