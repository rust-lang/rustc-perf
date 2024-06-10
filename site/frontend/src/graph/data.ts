export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

// Parameters used to filter graph data
export interface GraphsSelector {
  start: string;
  end: string;
  kind: GraphKind;
  stat: string;
  benchmark: string | null;
  scenario: string | null;
  profile: string | null;
}

export interface Series {
  points: [number];
  interpolated_indices: Set<number>;
}

// Graph data received from the server
export interface CompileGraphData {
  commits: Array<[number, string]>;
  // benchmark -> profile -> scenario -> series
  benchmarks: Dict<Dict<Dict<Series>>>;
}

export interface RuntimeGraphData {
  commits: Array<[number, string]>;
  // benchmark ->  series
  benchmarks: Dict<Series>;
}
