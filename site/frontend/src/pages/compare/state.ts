export interface CompareSelector {
  start: string;
  end: string;
  stat: string;
}

export interface BenchmarkDescription {
  name: string;
  category: string;
}

export interface ArtifactDescription {
  commit: string;
  date: string | null;
  pr: number | null;
  bootstrap: Dict<number>;
  bootstrap_total: number;
}

export interface Comparison {
  benchmark: string;
  profile: string;
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

  comparisons: [Comparison];
  new_errors: Array<[string, string]>;
  benchmark_data: [BenchmarkDescription];
}
