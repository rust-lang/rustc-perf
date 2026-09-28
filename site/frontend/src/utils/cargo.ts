import {Profile} from "../pages/compare/compile/common";

/**
 * Return a Cargo command that builds rustc-perf properly and then executes
 * the collector.
 * The returned command is meant to be followed by a `collector` command and its
 * arguments.
 */
export function cargo_collector_command(): string {
  return "cargo build --release -p collector && ./target/release/collector";
}

// Normalizes the backend name so that it can be used as a CLI argument for
// `rustc-perf`'s collector
export function normalizeBackendName(backend: string): string {
  // e.g. cranelift to Cranelift
  return kebabToPascalCase(backend);
}

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
