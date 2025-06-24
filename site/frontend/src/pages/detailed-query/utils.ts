import {
  chromeProfileUrl,
  processedSelfProfileRelativeUrl,
} from "../../self-profile";

export interface Selector {
  commit: string;
  base_commit: string | null;
  benchmark: string;
  scenario: string;
}

export interface ProfileElement {
  label: string;
  self_time: number;
  percent_total_time: number;
  number_of_cache_misses?: number;
  number_of_cache_hits?: number;
  invocation_count: number;
  blocked_time?: number;
  incremental_load_time: number;
}

export interface ArtifactSize {
  label: string;
  bytes: string;
}

export interface ProfileData {
  totals: ProfileElement;
  query_data: ProfileElement[];
  artifact_sizes: ArtifactSize[];
}

export interface SelfProfileResponse {
  profile: ProfileData;
  base_profile_delta?: ProfileData;
}

export function toSeconds(time: number): number {
  return time / 1000000000;
}

export function createDelta(
  to: number,
  delta: number,
  isIntegralDelta: boolean
): DeltaData {
  let from = to - delta;
  let pct = ((to - from) / from) * 100;
  if (from == to) {
    pct = 0;
  }

  return {
    delta,
    from,
    percentage: pct,
    isIntegral: isIntegralDelta,
  };
}

export function perfettoProfilerData(
  commit: string,
  benchmark: string,
  scenario: string
): {link: string; traceTitle: string} {
  const link = chromeProfileUrl(commit, benchmark, scenario);
  const traceTitle = `${benchmark}-${scenario} (${commit})`;
  return {link, traceTitle};
}

export function createTitleData(selector: Selector | null): {
  text: string;
  baseHref: string;
  selfHref: string;
} {
  if (!selector) return {text: "", baseHref: "", selfHref: ""};

  const state = selector;
  let text = `${state.commit.substring(0, 10)}: Self profile results for ${
    state.benchmark
  } run ${state.scenario}`;

  let baseHref = "";
  let selfHref = "";

  if (state.base_commit) {
    selfHref = `/detailed-query.html?commit=${state.commit}&scenario=${state.scenario}&benchmark=${state.benchmark}`;
    baseHref = `/detailed-query.html?commit=${state.base_commit}&scenario=${state.scenario}&benchmark=${state.benchmark}`;
  }

  return {text, baseHref, selfHref};
}

interface DownloadLinks {
  raw: string;
  flamegraph: string;
  crox: string;
  codegen: string;
  perfetto: {link: string; traceTitle: string};
  firefox: string;
}

export function createDownloadLinksData(selector: Selector | null): {
  baseLinks: DownloadLinks | null;
  newLinks: DownloadLinks;
  diffLink: string;
  localCommands: {
    base: string;
    new: string;
    diff: string;
  };
} {
  if (!selector)
    return {
      baseLinks: null,
      newLinks: {
        raw: "",
        flamegraph: "",
        crox: "",
        codegen: "",
        perfetto: {link: "", traceTitle: ""},
        firefox: "",
      },
      diffLink: "",
      localCommands: {base: "", new: "", diff: ""},
    };

  const state = selector;
  const profile = (b: string) =>
    b.endsWith("-opt")
      ? "Opt"
      : b.endsWith("-doc")
      ? "Doc"
      : b.endsWith("-debug")
      ? "Debug"
      : b.endsWith("-check")
      ? "Check"
      : "???";
  const benchName = (b: string) => b.replace(/-[^-]*$/, "");
  const scenarioFilter = (s: string) =>
    s == "full"
      ? "Full"
      : s == "incr-full"
      ? "IncrFull"
      : s == "incr-unchanged"
      ? "IncrUnchanged"
      : s.startsWith("incr-patched")
      ? "IncrPatched"
      : "???";

  const createLinks = (commit: string) => ({
    raw: `/perf/download-raw-self-profile?commit=${commit}&benchmark=${state.benchmark}&scenario=${state.scenario}`,
    flamegraph: processedSelfProfileRelativeUrl(
      commit,
      state.benchmark,
      state.scenario,
      "flamegraph"
    ),
    crox: processedSelfProfileRelativeUrl(
      commit,
      state.benchmark,
      state.scenario,
      "crox"
    ),
    codegen: processedSelfProfileRelativeUrl(
      commit,
      state.benchmark,
      state.scenario,
      "codegen-schedule"
    ),
    perfetto: perfettoProfilerData(commit, state.benchmark, state.scenario),
    firefox: `https://profiler.firefox.com/from-url/${encodeURIComponent(
      chromeProfileUrl(commit, state.benchmark, state.scenario)
    )}/marker-chart/?v=5`,
  });

  const baseLinks = state.base_commit ? createLinks(state.base_commit) : null;
  const newLinks = createLinks(state.commit);

  const diffLink = state.base_commit
    ? `/perf/processed-self-profile?commit=${state.commit}&base_commit=${state.base_commit}&benchmark=${state.benchmark}&scenario=${state.scenario}&type=codegen-schedule`
    : "";

  const localCommands = {
    base: state.base_commit
      ? `./target/release/collector profile_local cachegrind
                    +${state.base_commit} --exact-match ${benchName(
          state.benchmark
        )} --profiles
                ${profile(state.benchmark)} --scenarios ${scenarioFilter(
          state.scenario
        )}`
      : "",
    new: `./target/release/collector profile_local cachegrind
                +${state.commit} --exact-match ${benchName(
      state.benchmark
    )} --profiles
                ${profile(state.benchmark)} --scenarios ${scenarioFilter(
      state.scenario
    )}`,
    diff: state.base_commit
      ? `./target/release/collector profile_local cachegrind
                +${state.base_commit} --rustc2 +${
          state.commit
        } --exact-match ${benchName(state.benchmark)} --profiles
                ${profile(state.benchmark)} --scenarios ${scenarioFilter(
          state.scenario
        )}`
      : "",
  };

  return {baseLinks, newLinks, diffLink, localCommands};
}

export interface DeltaData {
  from: number;
  delta: number;
  percentage: number;
  isIntegral: boolean;
}

interface TableRowData {
  isTotal: boolean;
  label: string;
  timePercent: {value: number; formatted: string; title: string};
  timeSeconds: number;
  timeDelta: DeltaData | null;
  executions: number;
  executionsDelta: DeltaData | null;
  incrementalLoading: number;
  incrementalLoadingDelta: DeltaData | null;
}

export function createTableData(
  data: SelfProfileResponse | null
): TableRowData[] {
  if (!data) return [];
  const result: TableRowData[] = [];
  const profile = data.profile;
  const delta = data.base_profile_delta;

  // Add totals row first
  const totals = profile.totals;
  const totalsDelta = delta?.totals;
  result.push({
    isTotal: true,
    label: totals.label,
    timePercent:
      totals.percent_total_time < 0
        ? {
            value: -1,
            formatted: "-",
            title: "No wall-time stat collected for this run",
          }
        : {
            value: totals.percent_total_time,
            formatted: totals.percent_total_time.toFixed(2) + "%*",
            title: "% of cpu-time stat",
          },
    timeSeconds: toSeconds(totals.self_time),
    timeDelta: totalsDelta
      ? createDelta(
          toSeconds(totals.self_time),
          toSeconds(totalsDelta.self_time),
          false
        )
      : null,
    executions: totals.invocation_count,
    executionsDelta: totalsDelta
      ? createDelta(totals.invocation_count, totalsDelta.invocation_count, true)
      : null,
    incrementalLoading: toSeconds(totals.incremental_load_time),
    incrementalLoadingDelta: totalsDelta
      ? createDelta(
          toSeconds(totals.incremental_load_time),
          toSeconds(totalsDelta.incremental_load_time),
          false
        )
      : null,
  });

  // Add query data rows
  profile.query_data.forEach((query, idx) => {
    const queryDelta = delta?.query_data[idx];
    result.push({
      isTotal: false,
      label: query.label,
      timePercent:
        query.percent_total_time < 0
          ? {
              value: -1,
              formatted: "-",
              title: "No wall-time stat collected for this run",
            }
          : {
              value: query.percent_total_time,
              formatted: query.percent_total_time.toFixed(2) + "%",
              title: "",
            },
      timeSeconds: toSeconds(query.self_time),
      timeDelta: queryDelta
        ? createDelta(
            toSeconds(query.self_time),
            toSeconds(queryDelta.self_time),
            false
          )
        : null,
      executions: query.invocation_count,
      executionsDelta: queryDelta
        ? createDelta(query.invocation_count, queryDelta.invocation_count, true)
        : null,
      incrementalLoading: toSeconds(query.incremental_load_time),
      incrementalLoadingDelta: queryDelta
        ? createDelta(
            toSeconds(query.incremental_load_time),
            toSeconds(queryDelta.incremental_load_time),
            false
          )
        : null,
    });
  });

  return result;
}

export function createArtifactData(data: SelfProfileResponse | null): {
  name: string;
  size: string;
  sizeDelta: string;
}[] {
  if (!data) return [];
  return data.profile.artifact_sizes.map((artifact, idx) => ({
    name: artifact.label,
    size: artifact.bytes,
    sizeDelta: data?.base_profile_delta?.artifact_sizes[idx]?.bytes || "0",
  }));
}
