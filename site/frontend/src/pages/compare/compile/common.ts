import {BenchmarkFilter, CompareResponse, StatComparison} from "../types";
import {calculateComparison, TestCaseComparison} from "../data";

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
  backend: {
    llvm: boolean;
    cranelift: boolean;
  };
  category: {
    primary: boolean;
    secondary: boolean;
  };
  artifact: {
    binary: boolean;
    library: boolean;
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
  backend: {
    llvm: true,
    cranelift: true,
  },
  category: {
    primary: true,
    secondary: true,
  },
  artifact: {
    binary: true,
    library: true,
  },
};

export type Profile = "check" | "debug" | "opt" | "doc";
export type CodegenBackend = "llvm" | "cranelift";
export type Category = "primary" | "secondary";

export type CompileBenchmarkMap = Dict<CompileBenchmarkMetadata>;

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

export interface CompileBenchmarkComparison {
  benchmark: string;
  profile: Profile;
  scenario: string;
  backend: CodegenBackend;
  comparison: StatComparison;
}

export interface CompileTestCase {
  benchmark: string;
  profile: Profile;
  scenario: string;
  backend: CodegenBackend;
  category: Category;
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
    if (metadata?.binary === null) return true;

    const isBinary = metadata.binary;
    const isLibrary = !isBinary;
    if (isBinary && !filter.artifact.binary) return false;
    if (isLibrary && !filter.artifact.library) return false;

    return true;
  }

  function categoryFilter(category: Category) {
    if (category === "primary" && !filter.category.primary) return false;
    if (category === "secondary" && !filter.category.secondary) return false;
    return true;
  }

  function shouldShowTestCase(comparison: TestCaseComparison<CompileTestCase>) {
    const name = `${comparison.testCase.benchmark} ${comparison.testCase.profile} ${comparison.testCase.scenario} ${comparison.testCase.backend}`;
    const nameFilter = filter.name && filter.name.trim();
    const nameFiltered = !nameFilter || name.includes(nameFilter);

    return (
      profileFilter(comparison.testCase.profile) &&
      scenarioFilter(comparison.testCase.scenario) &&
      backendFilter(comparison.testCase.backend) &&
      categoryFilter(comparison.testCase.category) &&
      artifactFilter(benchmarkMap[comparison.testCase.benchmark] ?? null) &&
      nameFiltered
    );
  }

  let filteredComparisons = comparisons
    .map(
      (c: CompileBenchmarkComparison): TestCaseComparison<CompileTestCase> => {
        let testCase = {
          benchmark: c.benchmark,
          profile: c.profile,
          scenario: c.scenario,
          backend: c.backend,
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
  const benchmarks = {};
  for (const benchmark of data.compile_benchmark_metadata) {
    benchmarks[benchmark.name] = {...benchmark};
  }
  return benchmarks;
}
