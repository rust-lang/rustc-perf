import {BenchmarkFilter, StatComparison} from "../types";

export type RuntimeBenchmarkFilter = BenchmarkFilter;

export const defaultRuntimeFilter: RuntimeBenchmarkFilter = {
  name: null,
  nonRelevant: false,
  showRawData: false,
};

export interface RuntimeBenchmarkComparison {
  benchmark: string;
  comparison: StatComparison;
}
