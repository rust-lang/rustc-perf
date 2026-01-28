// Returns the URL to a measureme self-profile data, processed into the
// Chrome profiler format.
export function chromeProfileUrl(
  commit: string,
  benchmark: string,
  scenario: string,
  profile: string,
  backend: string,
  target: string
): string {
  const relativeUrl = processedSelfProfileRelativeUrl(
    commit,
    benchmark,
    scenario,
    profile,
    backend,
    target,
    "crox"
  );
  return window.location.origin + relativeUrl;
}

export function processedSelfProfileRelativeUrl(
  commit: string,
  benchmark: string,
  scenario: string,
  profile: string,
  backend: string,
  target: string,
  type: string
): string {
  return `/perf/processed-self-profile?commit=${commit}&benchmark=${benchmark}&profile=${profile}&scenario=${scenario}&backend=${backend}&target=${target}&type=${type}`;
}
