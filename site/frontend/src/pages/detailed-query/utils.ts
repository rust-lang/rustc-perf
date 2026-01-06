import {
  chromeProfileUrl,
  processedSelfProfileRelativeUrl,
} from "../../self-profile";
import {cargo_collector_command} from "../../utils/cargo";

export interface Selector {
  commit: string;
  base_commit: string | null;
  benchmark: string;
  scenario: string;
  backend: string;
  target: string;
}

export interface ProfileElement {
  label: string;
  self_time: number;
  percent_total_time: number;
  number_of_cache_misses: number;
  number_of_cache_hits: number;
  invocation_count: number;
  blocked_time: number;
  incremental_load_time: number;
}

export interface ProfileElementDelta {
  self_time: number;
  invocation_count: number;
  number_of_cache_hits: number;
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
  base_profile_delta?: ProfileDataDelta;
}

export interface ProfileDataDelta {
  totals: ProfileElementDelta;
  query_data: ProfileElementDelta[];
  artifact_sizes: ArtifactSize[];
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
    const args = `&scenario=${state.scenario}&benchmark=${state.benchmark}&backend=${state.backend}&target=${state.target}`;
    selfHref = `/detailed-query.html?commit=${state.commit}${args}`;
    baseHref = `/detailed-query.html?commit=${state.base_commit}${args}`;
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
      ? `${cargo_collector_command()} profile_local cachegrind
                    +${state.base_commit} --exact-match ${benchName(
          state.benchmark
        )} --profiles
                ${profile(state.benchmark)} --scenarios ${scenarioFilter(
          state.scenario
        )}`
      : "",
    new: `${cargo_collector_command()} profile_local cachegrind
                +${state.commit} --exact-match ${benchName(
      state.benchmark
    )} --profiles
                ${profile(state.benchmark)} --scenarios ${scenarioFilter(
      state.scenario
    )}`,
    diff: state.base_commit
      ? `${cargo_collector_command()} profile_local cachegrind
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

export interface TableRowData {
  isTotal: boolean;
  label: string;
  timePercent: {value: number; formatted: string; title: string};
  timeSeconds: number;
  timeDelta: DeltaData | null;
  executions: number;
  executionsDelta: DeltaData | null;
  cacheHits: number;
  cacheHitsDelta: DeltaData | null;
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
  result.push(createRowData(true, totals, totalsDelta));

  // Add query data rows
  profile.query_data.forEach((query, idx) => {
    const queryDelta = delta?.query_data[idx];
    result.push(createRowData(false, query, queryDelta));
  });

  return result;
}

function createRowData(
  isTotal: boolean,
  value: ProfileElement,
  delta: ProfileElementDelta | undefined
): TableRowData {
  return {
    isTotal,
    label: value.label,
    timePercent:
      value.percent_total_time < 0
        ? {
            value: -1,
            formatted: "-",
            title: "No wall-time stat collected for this run",
          }
        : {
            value: value.percent_total_time,
            formatted: value.percent_total_time.toFixed(2) + "%",
            title: "% of cpu-time stat",
          },
    timeSeconds: toSeconds(value.self_time),
    timeDelta: delta
      ? createDelta(
          toSeconds(value.self_time),
          toSeconds(delta.self_time),
          false
        )
      : null,
    executions: value.invocation_count,
    executionsDelta: delta
      ? createDelta(value.invocation_count, delta.invocation_count, true)
      : null,
    cacheHits: value.number_of_cache_hits,
    cacheHitsDelta: delta
      ? createDelta(
          value.number_of_cache_hits,
          delta.number_of_cache_hits,
          true
        )
      : null,
    incrementalLoading: toSeconds(value.incremental_load_time),
    incrementalLoadingDelta: delta
      ? createDelta(
          toSeconds(value.incremental_load_time),
          toSeconds(delta.incremental_load_time),
          false
        )
      : null,
  };
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
