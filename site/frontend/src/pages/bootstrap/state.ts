// Parameters used to filter bootstrap data
export interface BootstrapSelector {
  start: string;
  end: string;
  min_seconds: number;
}

// Bootstrap data received from the server
export interface BootstrapData {
  commits: [[number, string]];
  by_crate_build_times: Dict<[number]>;
  total_build_times: [number];
}
