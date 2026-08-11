import {capitalize} from "vue";
import {mapFromJSON, MapWrapper} from "../utils/map-wrapper.ts";

export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

declare const brand: unique symbol;
type Brand<T, Tag extends string> = T & {readonly [brand]: Tag};

/**
 * specific key type for ProfileSeries as we have to deal with different
 * lowercased/capitalized representations depending on the use case
 */
type ProfileName = Brand<string, "ProfileName">;

const KNOWN_PROFILES_TO_BE_LOWERCASED = new Set([
  "Check",
  "Debug",
  "Opt",
  "Doc",
]);

/**
 * The server returns profiles capitalized, so we need to match that
 * here, so that the graph code can find the expected profile.
 * @param raw custom string
 * @returns representation is always capitalized
 */
export function toProfileKey(raw: string): ProfileName {
  return capitalize(raw) as ProfileName;
}

export function profileKeyForGraphDisplay(profileKey: ProfileName): string {
  if (profileKey in KNOWN_PROFILES_TO_BE_LOWERCASED) {
    return profileKey.toLowerCase();
  }
  return profileKey;
}

// other keys just to be safe
type ScenarioName = Brand<string, "ScenarioName">;
type FrontendThreadsName = Brand<string, "FrontendThreadsName">;
type CompileBenchmarkName = Brand<string, "CompileBenchmarkName">;

// Parameters used to filter graph data
export interface GraphsSelector {
  start: string;
  end: string;
  kind: GraphKind;
  stat: string;
  benchmark: CompileBenchmarkName | string | null;
  scenario: ScenarioName | null;
  frontendThreads: FrontendThreadsName | null;
  profile: ProfileName | null;
  backend: string | null;
  target: string | null;
}

export interface Series {
  points: [number];
  interpolated_indices: Set<number>;
}

export class FrontendThreadsSeries extends MapWrapper<
  FrontendThreadsName,
  Series
> {
  static fromJSON(json: Dict<Series>): FrontendThreadsSeries {
    return new FrontendThreadsSeries(
      mapFromJSON(
        json,
        (k) => k as FrontendThreadsName,
        (s) => s
      )
    );
  }
}

export class ScenarioSeries extends MapWrapper<
  ScenarioName,
  FrontendThreadsSeries
> {
  static fromJSON(json: Dict<Dict<Series>>): ScenarioSeries {
    return new ScenarioSeries(
      mapFromJSON(
        json,
        (k) => k as ScenarioName,
        FrontendThreadsSeries.fromJSON
      )
    );
  }
}

export class ProfileSeries extends MapWrapper<ProfileName, ScenarioSeries> {
  // here we have to convert to the proper ProfileKey representation
  static fromJSON(json: Dict<Dict<Dict<Series>>>): ProfileSeries {
    return new ProfileSeries(
      mapFromJSON(json, toProfileKey, ScenarioSeries.fromJSON)
    );
  }
}

/**
 * benchmarks are of a fixed nested structure:
 *   - benchmark -> profile -> scenario -> frontend_threads -> series
 */
export class CompileBenchmarks extends MapWrapper<
  CompileBenchmarkName,
  ProfileSeries
> {
  static fromJSON(json: Dict<Dict<Dict<Dict<Series>>>>): CompileBenchmarks {
    return new CompileBenchmarks(
      mapFromJSON(
        json,
        (k) => k as CompileBenchmarkName,
        ProfileSeries.fromJSON
      )
    );
  }
}

/** Graph data received from the server */
export interface CompileGraphData {
  commits: Array<[number, string]>;
  benchmarks: CompileBenchmarks;
}

export interface RuntimeGraphData {
  commits: Array<[number, string]>;
  /** benchmark ->  series */
  benchmarks: Dict<Series>;
}
