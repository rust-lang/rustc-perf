export type BenchmarkRequestType = "Release" | "Master" | "Try";
export type BenchmarkRequestStatus = "Queued" | "InProgress" | "Completed";

export type BenchmarkRequest = {
  tag: string;
  pr: number | null;
  status: BenchmarkRequestStatus;
  requestType: BenchmarkRequestType;
  createdAt: string;
  completedAt: string | null;
  endEstimated: boolean;
  durationS: number | null;
  errors: Dict<string>;
};

export type BenchmarkJobStatus = "Queued" | "InProgress" | "Success" | "Failed";
export type BenchmarkJobKind = "compiletime" | "runtimeInProgress" | "rustc";

export type BenchmarkJob = {
  requestTag: string;
  kind: BenchmarkJobKind;
  target: string;
  backend: string;
  profile: string;
  benchmarkSet: number;
  createdAt: string;
  startedAt: string | null;
  completedAt: string | null;
  status: BenchmarkJobStatus;
  dequeCounter: number;
};

export type PastRequestDuration = {
  requestTag: String;
  jobDurationS: number;
};

export type CollectorConfig = {
  name: string;
  target: string;
  benchmarkSet: number;
  isActive: boolean;
  lastHeartbeatAt: string;
  dateAdded: string;
  commitSha: string;
  jobs: BenchmarkJob[];
  pastRequestDurations: PastRequestDuration[];
};

export type StatusResponse = {
  requests: BenchmarkRequest[];
  collectors: CollectorConfig[];
};

export function isJobComplete(job: BenchmarkJob): boolean {
  return job.status === "Failed" || job.status === "Success";
}

export function isRequestInProgress(req: BenchmarkRequest): boolean {
  return req.status === "InProgress";
}

export function tagLooksLikeSha(tag: string): boolean {
  return tag.length === 40;
}
