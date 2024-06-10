import {GraphKind, Series} from "../../../graph/data";
import {COMPARE_RUNTIME_DETAIL_GRAPHS_DATA_URL} from "../../../urls";
import {getJson} from "../../../utils/requests";
import {CachedDataLoader} from "../compile/table/utils";

// Runtime benchmark detail received from the server
export interface RuntimeDetailGraphs {
  commits: Array<[number, string]>;
  // One Series for each GraphKind in the RuntimeDetailSelector
  graphs: Series[];
}

export interface RuntimeDetailGraphsSelector {
  start: string;
  end: string;
  stat: string;
  benchmark: string;
  kinds: GraphKind[];
}

async function loadGraphsDetail(
  selector: RuntimeDetailGraphsSelector
): Promise<RuntimeDetailGraphs> {
  const params = {
    start: selector.start,
    end: selector.end,
    stat: selector.stat,
    benchmark: selector.benchmark,
    scenario: null,
    profile: null,
    kinds: selector.kinds.join(","),
  };
  return await getJson<RuntimeDetailGraphs>(
    COMPARE_RUNTIME_DETAIL_GRAPHS_DATA_URL,
    params
  );
}

export const RUNTIME_DETAIL_GRAPHS_RESOLVER: CachedDataLoader<
  RuntimeDetailGraphsSelector,
  RuntimeDetailGraphs
> = new CachedDataLoader(
  (key: RuntimeDetailGraphsSelector) =>
    `${key.benchmark};${key.start};${key.end};${key.stat};${key.kinds}`,
  loadGraphsDetail
);
