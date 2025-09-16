export type BenchmarkRequestType = "Release" | "Master" | "Try";
export type BenchmarkRequestStatus = "Queued" | "InProgress" | "Completed";

export type BenchmarkRequest = {
  tag: string;
  pr: number | null;
  status: BenchmarkRequestStatus;
  requestType: BenchmarkRequestType;
  createdAt: string;
  completedAt: string | null;
  durationS: number | null;
  errors: Dict<string>;
};

export type BenchmarkJobStatus = "Queued" | "InProgress" | "Success" | "Failed";

export type BenchmarkJob = {
  requestTag: string;
  target: string;
  backend: string;
  profile: string;
  benchmarkSet: number;
  createdAt: string;
  startedAt: string | null;
  completedAt: string | null;
  status: BenchmarkJobStatus;
  dequeCounter: number;
  estimatedCompletedAt: string | null;
};

export type CollectorConfig = {
  name: string;
  target: string;
  benchmarkSet: number;
  isActive: boolean;
  lastHeartbeatAt: string;
  dateAdded: string;
  jobs: BenchmarkJob[];
};

export type StatusResponse = {
  requests: BenchmarkRequest[];
  collectors: CollectorConfig[];
};
