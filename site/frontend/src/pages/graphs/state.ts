export type GraphKind = "raw" | "percentfromfirst" | "percentrelative";

// Parameters used to filter graph data
export interface GraphsSelector {
    start: string;
    end: string;
    kind: GraphKind;
    stat: string;
    benchmark: string | null;
    scenario: string | null;
    profile: string | null;
    // Exclude benchmarks whose name ends with this suffix
    exclude_suffix: string | null;
    // Only include benchmarks whose name ends with this suffix
    include_suffix: string | null;
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
