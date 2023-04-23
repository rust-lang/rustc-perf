export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

// Parameters used to filter graph data
export interface GraphsSelector {
    start: string;
    end: string;
    kind: GraphKind;
    stat: string;
}

export interface Series {
    points: [number],
    interpolated_indices: Set<number>
}

// Graph data received from the server
export interface GraphData {
    commits: [[number, string]],
    benchmarks: Dict<Dict<Dict<Series>>>,
}

export interface BenchmarkInfo {
    // Known statistic values from the DB
    stats: [string];
    // Last loaded run date
    as_of: string | null;
}
