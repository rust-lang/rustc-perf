import {GraphKind, Series} from "../../../../graph/data";
import {getJson} from "../../../../utils/requests";
import {
  COMPARE_COMPILE_DETAIL_GRAPHS_DATA_URL,
  COMPARE_COMPILE_DETAIL_SECTIONS_DATA_URL,
} from "../../../../urls";
import {CachedDataLoader} from "./utils";

export interface CompileDetailGraphsSelector {
  start: string;
  end: string;
  stat: string;
  benchmark: string;
  scenario: string;
  profile: string;
  kinds: GraphKind[];
}

// Compile benchmark detail received from the server
export interface CompileDetailGraphs {
  commits: Array<[number, string]>;
  // One Series for each GraphKind in the CompileDetailSelector
  graphs: Series[];
  sections_before: CompilationSections | null;
  sections_after: CompilationSections | null;
}

export interface CompileDetailSectionsSelector {
  start: string;
  end: string;
  benchmark: string;
  scenario: string;
  profile: string;
}

export interface CompileDetailSections {
  before: CompilationSections | null;
  after: CompilationSections | null;
}

export interface CompilationSection {
  name: string;
  value: number;
}

export interface CompilationSections {
  sections: CompilationSection[];
}

/**
 * Compile benchmark detail resolver that contains a cache of downloaded details.
 * This is important for Vue components that download the benchmark detail on mount.
 * Without a cache, they would download the detail each time they are destroyed
 * and recreated.
 * This is essentially a global variable, but it makes the code simpler and
 * since we currently don't have any unit tests, we don't really need to avoid
 * global variables that much. If needed, it could be provided to Vue components
 * from a parent via props or context.
 */
export const COMPILE_DETAIL_GRAPHS_RESOLVER: CachedDataLoader<
  CompileDetailGraphsSelector,
  CompileDetailGraphs
> = new CachedDataLoader(
  (key: CompileDetailGraphsSelector) =>
    `${key.benchmark};${key.profile};${key.scenario};${key.start};${key.end};${key.stat};${key.kinds}`,
  loadGraphsDetail
);

async function loadGraphsDetail(
  selector: CompileDetailGraphsSelector
): Promise<CompileDetailGraphs> {
  const params = {
    start: selector.start,
    end: selector.end,
    stat: selector.stat,
    benchmark: selector.benchmark,
    scenario: selector.scenario,
    profile: selector.profile,
    kinds: selector.kinds.join(","),
  };
  return await getJson<CompileDetailGraphs>(
    COMPARE_COMPILE_DETAIL_GRAPHS_DATA_URL,
    params
  );
}

// The same thing, but for sections
export const COMPILE_DETAIL_SECTIONS_RESOLVER: CachedDataLoader<
  CompileDetailSectionsSelector,
  CompileDetailSections
> = new CachedDataLoader(
  (key: CompileDetailGraphsSelector) =>
    `${key.benchmark};${key.profile};${key.scenario};${key.start};${key.end}`,
  loadSectionsDetail
);

async function loadSectionsDetail(
  selector: CompileDetailSectionsSelector
): Promise<CompileDetailSections> {
  const params = {
    start: selector.start,
    end: selector.end,
    benchmark: selector.benchmark,
    scenario: selector.scenario,
    profile: selector.profile,
  };
  return await getJson<CompileDetailSections>(
    COMPARE_COMPILE_DETAIL_SECTIONS_DATA_URL,
    params
  );
}
