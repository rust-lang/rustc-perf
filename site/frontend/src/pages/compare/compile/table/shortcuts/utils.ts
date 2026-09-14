import {Profile} from "../../common";

// Normalize profile from a test case to a CLI value for collector.
export function normalizeProfile(profile: Profile): string {
  // e.g. doc-json to DocJson
  return kebabToPascalCase(profile);
}
export function normalizeScenario(scenario: string): string {
  // Get rid of incr-patched: foo
  let start = scenario.split(":")[0];

  // e.g. incr-unchanged to IncrUnchanged
  return kebabToPascalCase(start);
}

// From https://stackoverflow.com/a/54651317/1107768.
function kebabToPascalCase(text: string): string {
  return text.replace(/(^\w|-\w)/g, clearAndUpper);
}

function clearAndUpper(text: string): string {
  return text.replace(/-/, "").toUpperCase();
}
