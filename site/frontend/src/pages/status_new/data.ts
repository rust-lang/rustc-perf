type BenchmarkRequestStatusComplete = {
  state: "completed";
  completedAt: string;
  duration: number; // time in milliseconds
};

type BenchmarkRequestStatusInProgress = {
  state: "in_progress";
};

type BenchmarkRequestStatusArtifactsReady = {
  state: "artifacts_ready";
};

export type BenchmarkRequestStatus =
  | BenchmarkRequestStatusComplete
  | BenchmarkRequestStatusInProgress
  | BenchmarkRequestStatusArtifactsReady;

type BenchmarkRequestTypeTry = {
  type: "Try";
  tag: string | null;
  parent_sha: string | null;
  pr: number;
};

type BenchmarkRequestTypeMaster = {
  type: "Master";
  tag: string;
  parent_sha: string;
  pr: number;
};

type BenchmarkRequestTypeRelease = {
  type: "Try";
  tag: string;
};

type BenchmarkRequestType =
  | BenchmarkRequestTypeTry
  | BenchmarkRequestTypeMaster
  | BenchmarkRequestTypeRelease;

export type BenchmarkRequestComplete = {
  status: BenchmarkRequestStatusComplete;
  requestType: BenchmarkRequestType;
  commitDate: string | null;
  createdAt: string | null;
  backends: string[];
  profiles: string;
  errors: string[];
};

export type BenchmarkRequestInProgress = {
  status: BenchmarkRequestStatusInProgress;
  requestType: BenchmarkRequestType;
  commitDate: string | null;
  createdAt: string | null;
  backends: string[];
  profiles: string;
  errors: string[];
};

export type BenchmarkRequestArtifactsReady = {
  status: BenchmarkRequestStatusArtifactsReady;
  requestType: BenchmarkRequestType;
  commitDate: string | null;
  createdAt: string | null;
  backends: string[];
  profiles: string;
  errors: string[];
};

export type BenchmarkRequest =
  | BenchmarkRequestComplete
  | BenchmarkRequestInProgress
  | BenchmarkRequestArtifactsReady;

export type BenchmarkJobStatusQueued = {
  state: "queued";
};

export type BenchmarkJobStatusInProgress = {
  state: "in_progress";
  startedAt: string;
  collectorName: string;
};

export type BenchmarkJobStatusFailed = {
  state: "failed";
  startedAt: string;
  completedAt: string;
  collectorName: string;
};

export type BenchmarkJobStatusSuccess = {
  state: "success";
  startedAt: string;
  completedAt: string;
  collectorName: string;
};

export type BenchmarkJobStatus =
  | BenchmarkJobStatusSuccess
  | BenchmarkJobStatusFailed
  | BenchmarkJobStatusInProgress
  | BenchmarkJobStatusQueued;

export type BenchmarkJob = {
  target: string;
  backend: string;
  profile: string;
  requestTag: string;
  benchmarkSet: number;
  createdAt: string;
  status: BenchmarkJobStatus;
  dequeCounter: number;
};

export type CollectorConfig = {
  name: string;
  target: string;
  benchmarkSet: number;
  isActive: boolean;
  lastHeartbeatAt: string;
  dateAdded: string;
};

export type StatusResponseInProgress = {
  request: BenchmarkRequestInProgress;
  jobs: BenchmarkJob[];
};

export type StatusResponse = {
  completed: BenchmarkRequestComplete[];
  inProgress: StatusResponseInProgress[];
  collectorConfigs: CollectorConfig[];
  queue: BenchmarkRequest[];
};
