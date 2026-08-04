import {mapFromJSON, MapWrapper} from "../utils/map-wrapper.ts";

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

export class FrontendThreadsSeries extends MapWrapper<Series> {
  static fromJSON(json: Dict<Series>): FrontendThreadsSeries {
    return new FrontendThreadsSeries(mapFromJSON(json, (s) => s));
  }
}

export class ScenarioSeries extends MapWrapper<FrontendThreadsSeries> {
  static fromJSON(json: Dict<Dict<Series>>): ScenarioSeries {
    return new ScenarioSeries(
      mapFromJSON(json, FrontendThreadsSeries.fromJSON)
    );
  }
}

export class ProfileSeries extends MapWrapper<ScenarioSeries> {
  static fromJSON(json: Dict<Dict<Dict<Series>>>): ProfileSeries {
    return new ProfileSeries(mapFromJSON(json, ScenarioSeries.fromJSON));
  }
}

export class Benchmarks extends MapWrapper<ProfileSeries> {
  static fromJSON(json: Dict<Dict<Dict<Dict<Series>>>>): Benchmarks {
    return new Benchmarks(mapFromJSON(json, ProfileSeries.fromJSON));
  }
}

// Graph data received from the server
export interface CompileGraphData {
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
