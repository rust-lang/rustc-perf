export interface DataFilter {
  name: string | null;
  nonRelevant: boolean;
  showRawData: boolean;
  profile: {
    check: boolean;
    debug: boolean;
    opt: boolean;
    doc: boolean;
  };
  scenario: {
    full: boolean;
    incrFull: boolean;
    incrUnchanged: boolean;
    incrPatched: boolean;
  };
  category: {
    primary: boolean;
    secondary: boolean;
  };
}

export type Profile = "check" | "debug" | "opt" | "doc";
export type Category = "primary" | "secondary";

export type BenchmarkMap = Dict<{category: Category}>;

export interface CompareSelector {
  start: string;
  end: string;
  stat: string;
}

export interface CompileBenchmarkDescription {
  name: string;
  category: Category;
}

export interface ArtifactDescription {
  commit: string;
  date: string | null;
  pr: number | null;
  bootstrap: Dict<number>;
  bootstrap_total: number;
}

export interface CompileBenchmarkComparison {
  benchmark: string;
  profile: Profile;
  scenario: string;
  is_relevant: boolean;
  significance_threshold: number;
  significance_factor: number;
  statistics: [number, number];
}

export interface CompareResponse {
  prev: string | null;
  next: string | null;
  is_contiguous: boolean;

  a: ArtifactDescription;
  b: ArtifactDescription;

  compile_comparisons: [CompileBenchmarkComparison];
  new_errors: Array<[string, string]>;
  compile_benchmark_data: [CompileBenchmarkDescription];
}

export enum Tab {
  CompileTime,
  Bootstrap,
}
