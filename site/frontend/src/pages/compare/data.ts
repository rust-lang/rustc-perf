import {
  BenchmarkMap,
  Category,
  CompareResponse,
  CompileBenchmarkComparison,
  DataFilter,
  Profile,
} from "./types";

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

export interface TestCase {
  benchmark: string;
  profile: Profile;
  scenario: string;
  category: Category;
  isRelevant: boolean;
  significanceFactor: number;
  significanceThreshold: number;
  datumA: number;
  datumB: number;
  percent: number;
}

export function computeTestCasesWithNonRelevant(
  filter: DataFilter,
  data: CompareResponse,
  benchmarkMap: BenchmarkMap
): TestCase[] {
  function profileFilter(profile: Profile): boolean {
    if (profile === "check") {
      return filter.profile.check;
    } else if (profile === "debug") {
      return filter.profile.debug;
    } else if (profile === "opt") {
      return filter.profile.opt;
    } else if (profile === "doc") {
      return filter.profile.doc;
    } else {
      return true;
    }
  }

  function scenarioFilter(scenario: string): boolean {
    if (scenario === "full") {
      return filter.scenario.full;
    } else if (scenario === "incr-full") {
      return filter.scenario.incrFull;
    } else if (scenario === "incr-unchanged") {
      return filter.scenario.incrUnchanged;
    } else if (scenario.startsWith("incr-patched")) {
      return filter.scenario.incrPatched;
    } else {
      // Unknown, but by default we should show things
      return true;
    }
  }

  function categoryFilter(category: Category) {
    if (category === "primary" && !filter.category.primary) return false;
    if (category === "secondary" && !filter.category.secondary) return false;
    return true;
  }

  function shouldShowTestCase(testCase: TestCase) {
    const name = `${testCase.benchmark} ${testCase.profile} ${testCase.scenario}`;
    const nameFilter = filter.name && filter.name.trim();
    const nameFiltered = !nameFilter || name.includes(nameFilter);

    return (
      profileFilter(testCase.profile) &&
      scenarioFilter(testCase.scenario) &&
      categoryFilter(testCase.category) &&
      nameFiltered
    );
  }

  let testCases = data.compile_comparisons
    .map((c: CompileBenchmarkComparison): TestCase => {
      const datumA = c.comparison.statistics[0];
      const datumB = c.comparison.statistics[1];

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
        benchmark: c.benchmark,
        profile: c.profile,
        scenario: c.scenario,
        category: (benchmarkMap[c.benchmark] || {}).category || "secondary",
        isRelevant: c.comparison.is_relevant,
        significanceFactor: c.comparison.significance_factor,
        significanceThreshold: c.comparison.significance_threshold * 100.0, // ensure the threshold is in %
        datumA,
        datumB,
        percent,
      };
    })
    .filter((tc) => shouldShowTestCase(tc));

  // Sort by name first, so that there is a canonical ordering
  // of test cases. This ensures the overall order is stable, even if
  // individual benchmarks have the same largestChange value.
  testCases.sort((a, b) => a.benchmark.localeCompare(b.benchmark));
  testCases.sort((a, b) => Math.abs(b.percent) - Math.abs(a.percent));

  return testCases;
}

export function filterNonRelevant(
  filter: DataFilter,
  cases: TestCase[]
): TestCase[] {
  if (filter.nonRelevant) {
    return cases;
  }
  return cases.filter((c) => c.isRelevant);
}

/**
 * Computes summaries of improvements, regressions and all changes from the
 * given `testCases`.
 */
export function computeSummary(testCases: TestCase[]): SummaryGroup {
  const regressions = {
    values: [],
    benchmarks: new Set(),
  };
  const improvements = {
    values: [],
    benchmarks: new Set(),
  };
  const all = {
    values: [],
    benchmarks: new Set(),
  };

  const handleTestCase = (items, testCase) => {
    items.benchmarks.add(testCase.benchmark);
    items.values.push(testCase.percent);
  };

  for (const testCase of testCases) {
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

export function computeBenchmarkMap(
  data: CompareResponse | null
): BenchmarkMap {
  if (data === null) return {};

  const benchmarks = {};
  for (const benchmark of data.compile_benchmark_data) {
    benchmarks[benchmark.name] = {
      category: benchmark.category,
    };
  }
  return benchmarks;
}
