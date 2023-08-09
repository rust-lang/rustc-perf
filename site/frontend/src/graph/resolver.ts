import {GraphData, GraphsSelector} from "./data";
import {loadGraphs} from "./api";

/**
 * Graph API resolver that contains a cache of downloaded graphs.
 * This is important for Vue components that download a graph on mount.
 * Without a cache, they would download a graph each time they are destroyed
 * and recreated.
 */
export class GraphResolver {
  private cache: Dict<GraphData> = {};

  public async loadGraph(selector: GraphsSelector): Promise<GraphData> {
    const key = `${selector.benchmark};${selector.profile};${selector.scenario};${selector.start};${selector.end};${selector.stat};${selector.kind}`;
    if (!this.cache.hasOwnProperty(key)) {
      this.cache[key] = await loadGraphs(selector);
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
export const GRAPH_RESOLVER = new GraphResolver();
