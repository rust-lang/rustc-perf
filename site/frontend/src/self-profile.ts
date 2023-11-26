// Returns the URL to a processed Crox format of a self-profile query.
export function croxTraceUrl(
  commit: string,
  benchmarkAndProfile: string,
  scenario: string
): string {
  const relativeUrl = processedSelfProfileUrl(
    commit,
    benchmarkAndProfile,
    scenario,
    "crox"
  );
  return window.location.origin + relativeUrl;
}

export function processedSelfProfileUrl(
  commit: string,
  benchmarkAndProfile: string,
  scenario: string,
  type: string
): string {
  return `/perf/processed-self-profile?commit=${commit}&benchmark=${benchmarkAndProfile}&scenario=${scenario}&type=${type}`;
}
