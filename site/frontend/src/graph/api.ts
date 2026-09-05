import {CompileGraphData, GraphsSelector} from "./data";
import {getJson} from "../utils/requests";
import {GRAPH_DATA_URL} from "../urls";

export async function loadGraphs(
  selector: GraphsSelector
): Promise<CompileGraphData> {
  const params: Dict<string> = {
    start: selector.start,
    end: selector.end,
    kind: selector.kind as string,
    stat: selector.stat,
    benchmark: selector.benchmark as string,
    scenario: selector.scenario as string,
    profile: selector.profile as string,
    backend: selector.backend as string,
    target: selector.target as string,
    frontend_threads: selector.frontend_threads as string,
  };
  return await getJson<CompileGraphData>(GRAPH_DATA_URL, params);
}
