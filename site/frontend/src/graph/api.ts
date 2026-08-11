import {
  CompileBenchmarks,
  CompileGraphData,
  GraphsSelector,
  Series,
} from "./data";
import {getJson} from "../utils/requests";
import {GRAPH_DATA_URL} from "../urls";

interface CompileGraphDataRaw {
  commits: Array<[number, string]>;
  benchmarks: Dict<Dict<Dict<Dict<Series>>>>;
}

export async function loadGraphs(
  selector: GraphsSelector
): Promise<CompileGraphData> {
  const params = {
    start: selector.start,
    end: selector.end,
    kind: selector.kind as string,
    stat: selector.stat,
    benchmark: selector.benchmark,
    scenario: selector.scenario,
    frontendThreads: selector.frontendThreads,
    profile: selector.profile,
    backend: selector.backend,
    target: selector.target,
  };
  const dict: Dict<string> = Object.entries(params).reduce(
    (acc, [key, value]) => {
      if (value !== null && value !== undefined) {
        acc[key] = value;
      }
      return acc;
    },
    {} as Dict<string>
  );
  const raw = await getJson<CompileGraphDataRaw>(GRAPH_DATA_URL, dict);
  return {
    commits: raw.commits,
    benchmarks: CompileBenchmarks.fromJSON(raw.benchmarks),
  };
}
