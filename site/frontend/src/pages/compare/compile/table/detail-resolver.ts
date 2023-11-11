import {GraphKind, Series} from "../../../../graph/data";
import {getJson} from "../../../../utils/requests";
import {COMPARE_COMPILE_DETAIL_DATA_URL} from "../../../../urls";

export interface CompileDetailSelector {
  start: string;
  end: string;
  stat: string;
  benchmark: string;
  scenario: string;
  profile: string;
  kinds: GraphKind[];
}

// Compile benchmark detail received from the server
export interface CompileDetail {
  commits: Array<[number, string]>;
  // One Series for each GraphKind in the CompileDetailSelector
  graphs: Series[];
}

/**
 * Compile benchmark detail resolver that contains a cache of downloaded details.
 * This is important for Vue components that download the benchmark detail on mount.
 * Without a cache, they would download the detail each time they are destroyed
 * and recreated.
 */
export class CompileBenchmarkDetailResolver {
  private cache: Dict<CompileDetail> = {};

  public async loadDetail(
    selector: CompileDetailSelector
  ): Promise<CompileDetail> {
    const key = `${selector.benchmark};${selector.profile};${selector.scenario};${selector.start};${selector.end};${selector.stat};${selector.kinds}`;
    if (!this.cache.hasOwnProperty(key)) {
      this.cache[key] = await loadDetail(selector);
    }

    return this.cache[key];
  }
}

/**
 * This is essentially a global variable, but it makes the code simpler and
 * since we currently don't have any unit tests, we don't really need to avoid
 * global variables that much. If needed, it could be provided to Vue components
 * from a parent via props or context.
 */
export const COMPILE_DETAIL_RESOLVER = new CompileBenchmarkDetailResolver();

async function loadDetail(
  selector: CompileDetailSelector
): Promise<CompileDetail> {
  const params = {
    start: selector.start,
    end: selector.end,
    stat: selector.stat,
    benchmark: selector.benchmark,
    scenario: selector.scenario,
    profile: selector.profile,
    kinds: selector.kinds.join(","),
  };
  return await getJson<CompileDetail>(COMPARE_COMPILE_DETAIL_DATA_URL, params);
}
