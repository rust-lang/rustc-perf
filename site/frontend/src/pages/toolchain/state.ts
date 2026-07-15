// Parameters used to filter toolchain-specific data
export interface ToolchainSelector {
  start: string;
  end: string;
  min_seconds: number;
}

// Bootstrap data received from the server
export interface ToolchainData {
  commits: [[number, string]];
  by_crate_build_times: Dict<[number]>;
  total_build_times: [number];
}
