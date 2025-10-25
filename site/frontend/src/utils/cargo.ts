/**
 * Return a Cargo command that builds rustc-perf properly and then executes
 * the collector.
 * The returned command is meant to be followed by a `collector` command and its
 * arguments.
 */
export function cargo_collector_command(): string {
  return "cargo build --release -p collector && ./target/release/collector";
}
