import {CompileGraphData, GraphsSelector} from "./data";
import {getJson} from "../utils/requests";
import {GRAPH_DATA_URL} from "../urls";

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
    parallel: selector.parallel,
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
  return await getJson<CompileGraphData>(GRAPH_DATA_URL, dict);
}
