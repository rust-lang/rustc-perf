import {BenchmarkFilter, CompareResponse, StatComparison} from "../types";
import {calculateComparison, TestCaseComparison} from "../data";
import {benchmarkNameMatchesFilter, targetMatchesFilter} from "../shared";
import {DEFAULT_COMPILE_TARGET_TRIPLE} from "../../../api";

export type CompileBenchmarkFilter = {
  profile: {
    check: boolean;
    debug: boolean;
    opt: boolean;
    doc: boolean;
    docJson: boolean;
  };
  scenario: {
    full: boolean;
    incrFull: boolean;
    incrUnchanged: boolean;
    incrPatched: boolean;
  };
  backend: {
    llvm: boolean;
    cranelift: boolean;
  };
  target: Target[];
  category: {
    primary: boolean;
    secondary: boolean;
  };
  artifact: {
    binary: boolean;
    library: boolean;
  };
  changes: {
    regressions: boolean;
    improvements: boolean;
  };
  selfCompareParameter: string | null;
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
    docJson: true,
  },
  scenario: {
    full: true,
    incrFull: true,
    incrUnchanged: true,
    incrPatched: true,
  },
  backend: {
    llvm: true,
    cranelift: true,
  },
  target: [DEFAULT_COMPILE_TARGET_TRIPLE],
  category: {
    primary: true,
    secondary: true,
  },
  artifact: {
    binary: true,
    library: true,
  },
  changes: {
    regressions: true,
    improvements: true,
  },
  selfCompareParameter: null,
};

export type Profile = "check" | "debug" | "opt" | "doc";
export type CodegenBackend = "llvm" | "cranelift";
export type Category = "primary" | "secondary";
export type Target = "x86_64-unknown-linux-gnu" | "aarch64-unknown-linux-gnu";

export type CompileBenchmarkMap = Dict<CompileBenchmarkMetadata>;

export type SelfCompareData = {
  // Which benchmark parameter are we comparing?
  parameter: "backend" | "target";
  // Which value of the parameter is the baseline?
  baseline: string;
};

export interface CargoProfileMetadata {
  debug: string | null;
  lto: string | null;
  codegen_units: number | null;
}

export interface CompileBenchmarkMetadata {
  name: string;
  category: Category;
  binary: boolean | null;
  iterations: number | null;
  release_profile: CargoProfileMetadata;
  dev_profile: CargoProfileMetadata;
}

export interface CompileBenchmarkParameters {
  benchmark: string;
  profile: Profile;
  scenario: string;
  backend: CodegenBackend;
  target: Target;
  // We treat the frontend thread count as a categorical variable, which is why
  // it is represented as a string, and not a number.
  frontend_threads: string;
}

export type CompileBenchmarkComparison = CompileBenchmarkParameters & {
  comparison: StatComparison;
};

export type CompileTestCase = CompileBenchmarkParameters & {
  category: Category;
};

// Add new attritbtues to this function when modifying the CompileTestCase!
export function testCaseKey(testCase: CompileTestCase): string {
  return `${testCase.benchmark};${testCase.profile};${testCase.scenario};${testCase.backend};${testCase.target};${testCase.frontend_threads};${testCase.category}`;
}

export function computeCompileComparisonsWithNonRelevant(
  filter: CompileBenchmarkFilter,
  comparisons: CompileBenchmarkComparison[],
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
    } else if (profile === "doc-json") {
      return filter.profile.docJson;
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

  function backendFilter(backend: string): boolean {
    if (backend === "llvm") {
      return filter.backend.llvm;
    } else if (backend === "cranelift") {
      return filter.backend.cranelift;
    } else {
      // Unknown, but by default we should show things
      return true;
    }
  }

  function artifactFilter(metadata: CompileBenchmarkMetadata | null): boolean {
    if (metadata === null || metadata?.binary === null) return true;

    const isBinary = metadata.binary;
    const isLibrary = !isBinary;
    if (isBinary && !filter.artifact.binary) return false;
    if (isLibrary && !filter.artifact.library) return false;

    return true;
  }

  function changeFilter(
    comparison: TestCaseComparison<CompileTestCase>
  ): boolean {
    const isImprovement = comparison.percent <= 0.0;
    if (isImprovement && !filter.changes.improvements) return false;
    if (!isImprovement && !filter.changes.regressions) return false;

    return true;
  }

  function categoryFilter(category: Category) {
    if (category === "primary" && !filter.category.primary) return false;
    if (category === "secondary" && !filter.category.secondary) return false;
    return true;
  }

  function shouldShowTestCase(comparison: TestCaseComparison<CompileTestCase>) {
    return (
      profileFilter(comparison.testCase.profile) &&
      scenarioFilter(comparison.testCase.scenario) &&
      backendFilter(comparison.testCase.backend) &&
      targetMatchesFilter(comparison.testCase.target, filter.target) &&
      categoryFilter(comparison.testCase.category) &&
      artifactFilter(benchmarkMap[comparison.testCase.benchmark] ?? null) &&
      changeFilter(comparison) &&
      benchmarkNameMatchesFilter(comparison.testCase.benchmark, filter.name)
    );
  }

  let filteredComparisons = comparisons
    .map(
      (c: CompileBenchmarkComparison): TestCaseComparison<CompileTestCase> => {
        let testCase: CompileTestCase = {
          benchmark: c.benchmark,
          profile: c.profile,
          scenario: c.scenario,
          backend: c.backend,
          target: c.target,
          frontend_threads: c.frontend_threads,
          category: (benchmarkMap[c.benchmark] || {}).category || "secondary",
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

export function createCompileBenchmarkMap(
  data: CompareResponse
): CompileBenchmarkMap {
  const benchmarks: CompileBenchmarkMap = {};
  for (const benchmark of data.compile_benchmark_metadata) {
    benchmarks[benchmark.name] = {...benchmark};
  }
  return benchmarks;
}

// Transform compile comparisons to compare treat the given benchmark
// parameter's baseline value as the previous commit data.
// Assumes that the data comes from the same commit.
export function transformDataForSelfComparison(
  comparisons: CompileBenchmarkComparison[],
  selfCompare: SelfCompareData
): CompileBenchmarkComparison[] {
  function computeKey(comparison: CompileBenchmarkComparison): string {
    // Create a key out of the comparison entry
    const object: any = {...comparison};
    // Remove metric comparison
    delete object["comparison"];
    // Remove the self-compare parameter
    delete object[selfCompare.parameter];

    const keys = Object.keys(object);
    keys.sort();
    return keys.map((k) => `${object[k]}`).join(";");
  }

  const baselineValues: Map<string, CompileBenchmarkComparison> = new Map();
  // Record baselines
  for (const comparison of comparisons) {
    const key = computeKey(comparison);
    if (comparison[selfCompare.parameter] === selfCompare.baseline) {
      baselineValues.set(key, comparison);
    }
  }

  // Construct new entries
  const result = [];
  for (const comparison of comparisons) {
    // Ignore comparison if it is the baseline
    if (comparison[selfCompare.parameter] === selfCompare.baseline) {
      continue;
    }
    // Find corresponding baseline for this comparison
    const key = computeKey(comparison);
    const baseline = baselineValues.get(key);
    // No baseline found
    if (baseline === undefined) {
      console.warn(
        `No baseline found for parameter ${selfCompare.parameter} and key ${key}.`
      );
      continue;
    }
    // Replace baseline entry
    const updated: CompileBenchmarkComparison = {
      ...comparison,
      comparison: {
        ...comparison.comparison,
        statistics: [
          // Baseline value
          baseline.comparison.statistics[0],
          // Current value
          comparison.comparison.statistics[1],
        ],
      },
    };
    result.push(updated);
  }
  return result;
}
