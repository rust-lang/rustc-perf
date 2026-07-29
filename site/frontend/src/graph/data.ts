export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

// Parameters used to filter graph data
export interface GraphsSelector {
  start: string;
  end: string;
  kind: GraphKind;
  stat: string;
  benchmark: string | null;
  scenario: string | null;
  frontendThreads: string | null;
  profile: string | null;
  backend: string | null;
  target: string | null;
}

export interface Series {
  points: [number];
  interpolated_indices: Set<number>;
}

class DictWrapper<T> {
  constructor(public readonly data: Dict<T>) {}

  get(key: string): T {
    return this.data[key];
  }

  keys(): string[] {
    return Object.keys(this.data);
  }

  toJSON(): Dict<T> {
    return this.data;
  }
}

export class FrontendThreadsSeries extends DictWrapper<Series> {}
export class ScenarioSeries extends DictWrapper<FrontendThreadsSeries> {}
export class ProfileSeries extends DictWrapper<ScenarioSeries> {}
export class Benchmarks extends DictWrapper<ProfileSeries> {}

// Graph data received from the server
export class CompileGraphData {
  commits: Array<[number, string]>;
  // benchmark -> profile -> parallel -> scenario -> series
  // WARNING: now uses new layout:
  // benchmark -> profile -> scenario -> frontend_threads -> series
  benchmarks: Benchmarks;
}

export interface RuntimeGraphData {
  commits: Array<[number, string]>;
  // benchmark ->  series
  benchmarks: Dict<Series>;
}
