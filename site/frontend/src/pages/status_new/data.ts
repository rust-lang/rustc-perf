export const BenchmarkRequestCompleteStr = "completed";
export const BenchmarkRequestInProgressStr = "in_progress";
export const BenchmarkRequestArtifactsReadyStr = "artifacts_ready";

type BenchmarkRequestStatusComplete = {
  state: typeof BenchmarkRequestCompleteStr;
  completedAt: string;
  duration_s: number;
};

type BenchmarkRequestStatusInProgress = {
  state: typeof BenchmarkRequestInProgressStr;
};

type BenchmarkRequestStatusArtifactsReady = {
  state: typeof BenchmarkRequestArtifactsReadyStr;
};

export type BenchmarkRequestStatus =
  | BenchmarkRequestStatusComplete
  | BenchmarkRequestStatusInProgress
  | BenchmarkRequestStatusArtifactsReady;

export const TryCommit = "Try";
export const MasterCommit = "Master";
export const ReleaseCommit = "Release";

type BenchmarkRequestTypeTry = {
  type: typeof TryCommit;
  tag: string | null;
  parent_sha: string | null;
  pr: number;
};

type BenchmarkRequestTypeMaster = {
  type: typeof MasterCommit;
  tag: string;
  parent_sha: string;
  pr: number;
};

type BenchmarkRequestTypeRelease = {
  type: typeof ReleaseCommit;
  tag: string;
};

export type BenchmarkRequestTypeStr =
  | typeof ReleaseCommit
  | typeof MasterCommit
  | typeof TryCommit;

export type BenchmarkRequestType =
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

export const BenchmarkJobQueued = "queued";
export const BenchmarkJobInProgress = "in_progres";
export const BenchmarkJobFailed = "failed";
export const BenchmarkJobSuccess = "success";

export type BenchmarkJobStatusQueued = {
  state: typeof BenchmarkJobQueued;
};

export type BenchmarkJobStatusInProgress = {
  state: typeof BenchmarkJobInProgress;
  startedAt: string;
  collectorName: string;
};

export type BenchmarkJobStatusFailed = {
  state: typeof BenchmarkJobFailed;
  startedAt: string;
  completedAt: string;
  collectorName: string;
};

export type BenchmarkJobStatusSuccess = {
  state: typeof BenchmarkJobSuccess;
  startedAt: string;
  completedAt: string;
  collectorName: string;
};

export type BenchmarkJobStatusStr =
  | typeof BenchmarkJobQueued
  | typeof BenchmarkJobInProgress
  | typeof BenchmarkJobFailed
  | typeof BenchmarkJobSuccess;

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

export type CollectorInfo = {
  config: CollectorConfig;
  jobIds: number[];
};

export type StatusResponse = {
  queueRequestTags: string[];
  requestsMap: Dict<BenchmarkRequest>;
  jobMap: Dict<BenchmarkJob>;
  collectorWorkMap: Dict<CollectorInfo>;
  tagToJobs: Dict<number[]>;
};
