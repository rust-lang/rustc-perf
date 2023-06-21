import {BenchmarkFilter, StatComparison} from "./types";

export interface Summary {
  count: number;
  benchmarks: number;
  average: number;
  range: Array<number>;
}

export interface SummaryGroup {
  improvements: Summary;
  regressions: Summary;
  all: Summary;
}

export interface TestCaseComparison<Case> {
  testCase: Case;
  isRelevant: boolean;
  significanceFactor: number;
  significanceThreshold: number;
  datumA: number;
  datumB: number;
  percent: number;
}

export function calculateComparison<Case>(
  comparison: StatComparison,
  testCase: Case
): TestCaseComparison<Case> {
  const datumA = comparison.statistics[0];
  const datumB = comparison.statistics[1];

  // In the vast majority of cases, we can do the proportional change calculation. However, some
  // metrics can be zero. If the initial value is 0, we can't compute the new value as a
  // percentage change of the old one. If both values are 0, we can say the change is also 0%.
  // If the new value is not 0, the percentage is not really meaningful, but we can say it's 100%.
  let percent;
  if (datumA === 0) {
    if (datumB === 0) {
      percent = 0;
    } else {
      percent = 100;
    }
  } else {
    percent = 100 * ((datumB - datumA) / datumA);
  }

  return {
    testCase,
    isRelevant: comparison.is_relevant,
    significanceFactor: comparison.significance_factor,
    significanceThreshold: comparison.significance_threshold * 100.0, // ensure the threshold is in %
    datumA,
    datumB,
    percent,
  };
}

export function filterNonRelevant<Case>(
  filter: BenchmarkFilter,
  cases: TestCaseComparison<Case>[]
): TestCaseComparison<Case>[] {
  if (filter.nonRelevant) {
    return cases;
  }
  return cases.filter((c) => c.isRelevant);
}

/**
 * Computes summaries of improvements, regressions and all changes from the
 * given `testCases`.
 */
export function computeSummary<Case extends {benchmark: string}>(
  comparisons: TestCaseComparison<Case>[]
): SummaryGroup {
  const regressions = {
    values: [],
    benchmarks: new Set(),
  };
  const improvements: typeof regressions = {
    values: [],
    benchmarks: new Set(),
  };
  const all: typeof regressions = {
    values: [],
    benchmarks: new Set(),
  };

  const handleTestCase = (
    items: typeof regressions,
    comparison: TestCaseComparison<Case>
  ) => {
    items.benchmarks.add(comparison.testCase.benchmark);
    items.values.push(comparison.percent);
  };

  for (const testCase of comparisons) {
    if (testCase.percent > 0) {
      handleTestCase(regressions, testCase);
    } else if (testCase.percent < 0) {
      handleTestCase(improvements, testCase);
    }
    handleTestCase(all, testCase);
  }

  const computeSummary = (data): Summary => {
    const values = data.values;
    const benchmarks = data.benchmarks;

    const count = values.length;
    let range: Array<number> = [];
    if (count > 0) {
      range = [Math.min.apply(null, values), Math.max.apply(null, values)];
    }

    const sum = values.reduce((acc, item) => acc + item, 0);
    const average = sum / Math.max(count, 1);

    return {
      count,
      benchmarks: benchmarks.size,
      average,
      range,
    };
  };

  return {
    improvements: computeSummary(improvements),
    regressions: computeSummary(regressions),
    all: computeSummary(all),
  };
}
