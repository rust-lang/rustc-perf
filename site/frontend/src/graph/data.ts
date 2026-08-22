import {capitalize} from "vue";
import {mapFromJSON, MapWrapper} from "../utils/map-wrapper.ts";

export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

declare const brand: unique symbol;
type Brand<T, Tag extends string> = T & {readonly [brand]: Tag};

/**
 * specific key type for ProfileSeries as we have to deal with different
 * lowercased/capitalized representations depending on the use case
 */
export type Profile = Brand<string, "Profile">;

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
export function toProfile(raw: string): Profile {
  return capitalize(raw) as Profile;
}

export function profileForGraphDisplay(profile: Profile): string {
  if (profile in KNOWN_PROFILES_TO_BE_LOWERCASED) {
    return profile.toLowerCase();
  }
  return profile;
}

// other keys just to be safe
export type Scenario = Brand<string, "Scenario">;
export function toScenario(raw: string): Scenario {
  return raw as Scenario;
}
export type FrontendThreads = Brand<string, "FrontendThreads">;
export function toFrontendThreads(raw: string): FrontendThreads {
  return raw as FrontendThreads;
}
export type CompileBenchmark = Brand<string, "CompileBenchmark">;
export function toCompileBenchmark(raw: string): CompileBenchmark {
  return raw as CompileBenchmark;
}

// Parameters used to filter graph data
export interface GraphsSelector {
  start: string;
  end: string;
  kind: GraphKind;
  stat: string;
  benchmark: CompileBenchmark | string | null;
  scenario: Scenario | null;
  frontendThreads: FrontendThreads | null;
  profile: Profile | null;
  backend: string | null;
  target: string | null;
}

export interface Series {
  points: [number];
  interpolated_indices: Set<number>;
}

export class FrontendThreadsSeries extends MapWrapper<FrontendThreads, Series> {
  static fromJSON(json: Dict<Series>): FrontendThreadsSeries {
    return new FrontendThreadsSeries(
      mapFromJSON(json, toFrontendThreads, (s) => s)
    );
  }
}

export class ScenarioSeries extends MapWrapper<
  Scenario,
  FrontendThreadsSeries
> {
  static fromJSON(json: Dict<Dict<Series>>): ScenarioSeries {
    return new ScenarioSeries(
      mapFromJSON(json, toScenario, FrontendThreadsSeries.fromJSON)
    );
  }
}

export class ProfileSeries extends MapWrapper<Profile, ScenarioSeries> {
  // here we have to convert to the proper ProfileKey representation
  static fromJSON(json: Dict<Dict<Dict<Series>>>): ProfileSeries {
    return new ProfileSeries(
      mapFromJSON(json, toProfile, ScenarioSeries.fromJSON)
    );
  }
}

/**
 * benchmarks are of a fixed nested structure:
 *   - benchmark -> profile -> scenario -> frontend_threads -> series
 */
export class CompileBenchmarkSeries extends MapWrapper<
  CompileBenchmark,
  ProfileSeries
> {
  static fromJSON(
    json: Dict<Dict<Dict<Dict<Series>>>>
  ): CompileBenchmarkSeries {
    return new CompileBenchmarkSeries(
      mapFromJSON(json, toCompileBenchmark, ProfileSeries.fromJSON)
    );
  }
}

/** Graph data received from the server */
export interface CompileGraphData {
  commits: Array<[number, string]>;
  benchmarks: CompileBenchmarkSeries;
}

export interface RuntimeGraphData {
  commits: Array<[number, string]>;
  /** benchmark ->  series */
  benchmarks: Dict<Series>;
}
