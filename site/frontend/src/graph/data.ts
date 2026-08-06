import {capitalize} from "vue";
import {mapFromJSON, MapWrapper} from "../utils/map-wrapper.ts";

declare const brand: unique symbol;

export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

/**
 * specific key type for ProfileSeries as we have to deal with different
 * lowercased/capitalized representations depending on the use case
 */
type ProfileKey = string & {readonly [brand]: "ProfileKey"};
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
export function toProfileKey(raw: string): ProfileKey {
  return capitalize(raw) as ProfileKey;
}

export function profileKeyForGraphDisplay(profileKey: ProfileKey): string {
  if (profileKey in KNOWN_PROFILES_TO_BE_LOWERCASED) {
    return profileKey.toLowerCase();
  }
  return profileKey;
}

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

export class FrontendThreadsSeries extends MapWrapper<string, Series> {
  static fromJSON(json: Dict<Series>): FrontendThreadsSeries {
    return new FrontendThreadsSeries(mapFromJSON(json, (s) => s));
  }
}

export class ScenarioSeries extends MapWrapper<string, FrontendThreadsSeries> {
  static fromJSON(json: Dict<Dict<Series>>): ScenarioSeries {
    return new ScenarioSeries(
      mapFromJSON(json, FrontendThreadsSeries.fromJSON)
    );
  }
}

export class ProfileSeries extends MapWrapper<ProfileKey, ScenarioSeries> {
  // here we have to convert to the proper ProfileKey representation
  static fromJSON(json: Dict<Dict<Dict<Series>>>): ProfileSeries {
    let sourceMap = mapFromJSON(json, ScenarioSeries.fromJSON);
    let resultMap: Map<ProfileKey, ScenarioSeries> = new Map();
    for (const [k, v] of sourceMap.entries()) {
      resultMap.set(toProfileKey(k), v);
    }
    return new ProfileSeries(resultMap);
  }
}

export class Benchmarks extends MapWrapper<string, ProfileSeries> {
  static fromJSON(json: Dict<Dict<Dict<Dict<Series>>>>): Benchmarks {
    return new Benchmarks(mapFromJSON(json, ProfileSeries.fromJSON));
  }
}

/** Graph data received from the server */
export interface CompileGraphData {
  commits: Array<[number, string]>;
  /**
   * benchmarks are of a fixed nested structure:
   *   - benchmark -> profile -> scenario -> frontend_threads -> series
   */
  benchmarks: Benchmarks;
}

export interface RuntimeGraphData {
  commits: Array<[number, string]>;
  /** benchmark ->  series */
  benchmarks: Dict<Series>;
}
