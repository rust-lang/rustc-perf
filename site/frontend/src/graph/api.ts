import {GraphData, GraphsSelector} from "./data";
import {getJson} from "../utils/requests";
import {GRAPH_DATA_URL} from "../urls";

export async function loadGraphs(selector: GraphsSelector): Promise<GraphData> {
  const params = {
    start: selector.start,
    end: selector.end,
    kind: selector.kind as string,
    stat: selector.stat,
    benchmark: selector.benchmark,
    scenario: selector.scenario,
    profile: selector.profile,
  };
  return await getJson<GraphData>(GRAPH_DATA_URL, params);
}
