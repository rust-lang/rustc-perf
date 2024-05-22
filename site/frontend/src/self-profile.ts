// Returns the URL to a measureme self-profile data, processed into the

import {CompileTestCase} from "./pages/compare/compile/common";
import {ArtifactDescription} from "./pages/compare/types";

// Chrome profiler format.
export function chromeProfileUrl(
  commit: string,
  benchmarkAndProfile: string,
  scenario: string
): string {
  const relativeUrl = processedSelfProfileRelativeUrl(
    commit,
    benchmarkAndProfile,
    scenario,
    "crox"
  );
  return window.location.origin + relativeUrl;
}

export function processedSelfProfileRelativeUrl(
  commit: string,
  benchmarkAndProfile: string,
  scenario: string,
  type: string
): string {
  return `/perf/processed-self-profile?commit=${commit}&benchmark=${benchmarkAndProfile}&scenario=${scenario}&type=${type}`;
}

export const catapultUrl = (
  artifact: ArtifactDescription,
  testCase: CompileTestCase
) => {
  const {commit} = artifact;
  const {benchmark, profile, scenario} = testCase;
  return `/perf/self-profile-viewer?commit=${commit}&benchmark=${benchmark}-${profile.toLowerCase()}&scenario=${scenario}&type=crox`;
};
