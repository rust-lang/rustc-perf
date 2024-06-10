import {BenchmarkFilter, StatComparison} from "../types";
import {calculateComparison, TestCaseComparison} from "../data";
import {benchmarkNameMatchesFilter} from "../shared";

export interface RuntimeTestCase {
  benchmark: string;
}

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

export function computeRuntimeComparisonsWithNonRelevant(
  filter: RuntimeBenchmarkFilter,
  comparisons: RuntimeBenchmarkComparison[]
): TestCaseComparison<RuntimeTestCase>[] {
  function shouldShowTestCase(
    comparison: TestCaseComparison<RuntimeTestCase>
  ): boolean {
    return benchmarkNameMatchesFilter(
      comparison.testCase.benchmark,
      filter.name
    );
  }

  let filteredComparisons = comparisons
    .map(
      (c: RuntimeBenchmarkComparison): TestCaseComparison<RuntimeTestCase> => {
        let testCase = {
          benchmark: c.benchmark,
        };
        return calculateComparison(c.comparison, testCase);
      }
    )
    .filter((tc) => shouldShowTestCase(tc));

  // Sort by name first, so that there is a canonical ordering
  // of test cases. This ensures the overall order is stable, even if
  // individual benchmarks have the same largestChange value.
  filteredComparisons.sort((a, b) =>
    a.testCase.benchmark.localeCompare(b.testCase.benchmark)
  );
  filteredComparisons.sort((a, b) => Math.abs(b.percent) - Math.abs(a.percent));

  return filteredComparisons;
}

export function testCaseKey(testCase: RuntimeTestCase): string {
  return `${testCase.benchmark}`;
}
