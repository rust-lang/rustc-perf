import {BenchmarkFilter, CompareResponse, StatComparison} from "../types";
import {TestCaseComparison} from "../data";

export type CompileBenchmarkFilter = {
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
} & BenchmarkFilter;
export const defaultCompileFilter: CompileBenchmarkFilter = {
  name: null,
  nonRelevant: false,
  showRawData: false,
  profile: {
    check: true,
    debug: true,
    opt: true,
    doc: true,
  },
  scenario: {
    full: true,
    incrFull: true,
    incrUnchanged: true,
    incrPatched: true,
  },
  category: {
    primary: true,
    secondary: true,
  },
};

export type Profile = "check" | "debug" | "opt" | "doc";
export type Category = "primary" | "secondary";

export type CompileBenchmarkMap = Dict<{category: Category}>;

export interface CompileBenchmarkDescription {
  name: string;
  category: Category;
}

export interface CompileBenchmarkComparison {
  benchmark: string;
  profile: Profile;
  scenario: string;
  comparison: StatComparison;
}

export interface CompileTestCase {
  benchmark: string;
  profile: Profile;
  scenario: string;
  category: Category;
}

export function computeCompileComparisonsWithNonRelevant(
  filter: CompileBenchmarkFilter,
  data: CompareResponse,
  benchmarkMap: CompileBenchmarkMap
): TestCaseComparison<CompileTestCase>[] {
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

  function shouldShowTestCase(comparison: TestCaseComparison<CompileTestCase>) {
    const name = `${comparison.testCase.benchmark} ${comparison.testCase.profile} ${comparison.testCase.scenario}`;
    const nameFilter = filter.name && filter.name.trim();
    const nameFiltered = !nameFilter || name.includes(nameFilter);

    return (
      profileFilter(comparison.testCase.profile) &&
      scenarioFilter(comparison.testCase.scenario) &&
      categoryFilter(comparison.testCase.category) &&
      nameFiltered
    );
  }

  let testCases = data.compile_comparisons
    .map(
      (c: CompileBenchmarkComparison): TestCaseComparison<CompileTestCase> => {
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
          testCase: {
            benchmark: c.benchmark,
            profile: c.profile,
            scenario: c.scenario,
            category: (benchmarkMap[c.benchmark] || {}).category || "secondary",
          },
          isRelevant: c.comparison.is_relevant,
          significanceFactor: c.comparison.significance_factor,
          significanceThreshold: c.comparison.significance_threshold * 100.0, // ensure the threshold is in %
          datumA,
          datumB,
          percent,
        };
      }
    )
    .filter((tc) => shouldShowTestCase(tc));

  // Sort by name first, so that there is a canonical ordering
  // of test cases. This ensures the overall order is stable, even if
  // individual benchmarks have the same largestChange value.
  testCases.sort((a, b) =>
    a.testCase.benchmark.localeCompare(b.testCase.benchmark)
  );
  testCases.sort((a, b) => Math.abs(b.percent) - Math.abs(a.percent));

  return testCases;
}

export function createCompileBenchmarkMap(
  data: CompareResponse
): CompileBenchmarkMap {
  const benchmarks = {};
  for (const benchmark of data.compile_benchmark_data) {
    benchmarks[benchmark.name] = {
      category: benchmark.category,
    };
  }
  return benchmarks;
}
