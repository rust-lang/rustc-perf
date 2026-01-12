//! This module serves for defining the benchmark sets that are used by individual collectors.
//! Each benchmark set is identified by a pair (target, ID); each target has a specific number
//! of IDs.
//! The union of all benchmark sets for a given target should represent all the benchmarks that
//! should be executed on a master/try artifact.
//! Release artifacts do not participate in the benchmark set splitting and are always executed
//! on a single collector per target.

mod compile_benchmarks;

use crate::compile::benchmark::target::Target;
use crate::compile::benchmark::BenchmarkName;

/// Represents a single set of master/try benchmarks.
#[derive(Debug)]
pub struct BenchmarkSetId {
    target: Target,
    index: u32,
}

impl BenchmarkSetId {
    pub fn new(target: Target, index: u32) -> Self {
        Self { target, index }
    }
}

/// Represents a subset of benchmarks that should be benchmarked by a single collector.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BenchmarkSetMember {
    /// Benchmark a specific compile-time benchmark
    CompileBenchmark(BenchmarkName),
}

#[derive(Debug)]
pub struct BenchmarkSet {
    members: Vec<BenchmarkSetMember>,
}

impl BenchmarkSet {
    pub fn members(&self) -> &[BenchmarkSetMember] {
        &self.members
    }
}

pub const BENCHMARK_SET_RUNTIME_BENCHMARKS: u32 = 0;
pub const BENCHMARK_SET_RUSTC: u32 = 0;

/// Return all benchmark sets for the given target.
pub fn get_benchmark_sets_for_target(target: Target) -> Vec<BenchmarkSet> {
    use compile_benchmarks::*;

    fn compile(name: &str) -> BenchmarkSetMember {
        BenchmarkSetMember::CompileBenchmark(BenchmarkName::from(name))
    }

    let stable = vec![
        compile(CARGO),
        compile(ENCODING),
        compile(FUTURES),
        compile(HTML5EVER),
        compile(INFLATE),
        compile(PISTON_IMAGE),
        compile(REGEX),
        compile(SYN),
        compile(TOKIO_WEBPUSH_SIMPLE),
    ];

    match target {
        Target::X86_64UnknownLinuxGnu => {
            // Set 0 automatically runs runtime benchmarks and the rustc benchmark, so it should
            // receive less compile-time benchmarks to balance.
            // Set 0 also runs all stable benchmarks, but those are separate and we don't
            // really care about benchmarking "performance" for them, so we don't bother splitting
            // them up.
            let set_0 = vec![
                compile(AWAIT_CALL_TREE),
                compile(CARGO_0_87_1),
                compile(BITMAPS_3_2_1),
                compile(BITMAPS_3_2_1_NEW_SOLVER),
                compile(CLAP_DERIVE_4_5_32),
                compile(COERCIONS),
                compile(CRANELIFT_CODEGEN_0_119_0),
                compile(CTFE_STRESS_5),
                compile(DEEP_VECTOR),
                compile(DEEPLY_NESTED_MULTI),
                compile(DERIVE),
                compile(DIESEL_2_2_10),
                compile(EXTERNS),
                compile(EZA_0_21_2),
            ];
            let set_1 = vec![
                compile(HELLOWORLD),
                compile(HELLOWORLD_TINY),
                compile(HTML5EVER_0_31_0),
                compile(HTML5EVER_0_31_0_NEW_SOLVER),
                compile(HYPER_1_6_0),
                compile(IMAGE_0_25_6),
                compile(INCLUDE_BLOB),
                compile(ISSUE_46449),
                compile(ISSUE_58319),
                compile(ISSUE_88862),
                compile(LARGE_WORKSPACE),
                compile(LIBC_0_2_172),
                compile(MANY_ASSOC_ITEMS),
                compile(MATCH_STRESS),
                compile(NALGEBRA_0_33_0),
                compile(NALGEBRA_0_33_0_NEW_SOLVER),
                compile(PROJECTION_CACHING),
                compile(REGEX_AUTOMATA_0_4_8),
                compile(REGRESSION_31157),
                compile(RIPGREP_14_1_1),
                compile(RIPGREP_14_1_1_TINY),
                compile(SERDE_1_0_219),
                compile(SERDE_1_0_219_NEW_SOLVER),
                compile(SERDE_1_0_219_THREADS4),
                compile(SERDE_DERIVE_1_0_219),
                compile(STM32F4_0_15_1),
                compile(SYN_2_0_101),
                compile(SYN_2_0_101_NEW_SOLVER),
                compile(TOKEN_STREAM_STRESS),
                compile(TT_MUNCHER),
                compile(TUPLE_STRESS),
                compile(TYPENUM_1_18_0),
                compile(UCD),
                compile(UNICODE_NORMALIZATION_0_1_24),
                compile(UNIFY_LINEARLY),
                compile(UNUSED_WARNINGS),
                compile(WF_PROJECTION_STRESS_65510),
                compile(WG_GRAMMAR),
            ];
            vec![
                BenchmarkSet {
                    members: stable.into_iter().chain(set_0).collect(),
                },
                BenchmarkSet { members: set_1 },
            ]
        }
        Target::AArch64UnknownLinuxGnu => {
            let set = vec![
                compile(AWAIT_CALL_TREE),
                compile(CARGO_0_87_1),
                compile(BITMAPS_3_2_1),
                compile(BITMAPS_3_2_1_NEW_SOLVER),
                compile(CLAP_DERIVE_4_5_32),
                compile(COERCIONS),
                compile(CRANELIFT_CODEGEN_0_119_0),
                compile(CTFE_STRESS_5),
                compile(DEEP_VECTOR),
                compile(DEEPLY_NESTED_MULTI),
                compile(DERIVE),
                compile(DIESEL_2_2_10),
                compile(EXTERNS),
                compile(EZA_0_21_2),
                // We split here on x86_64 for the two machines
                compile(HELLOWORLD),
                compile(HELLOWORLD_TINY),
                compile(HTML5EVER_0_31_0),
                compile(HTML5EVER_0_31_0_NEW_SOLVER),
                compile(HYPER_1_6_0),
                compile(IMAGE_0_25_6),
                compile(INCLUDE_BLOB),
                compile(ISSUE_46449),
                compile(ISSUE_58319),
                compile(ISSUE_88862),
                compile(LARGE_WORKSPACE),
                compile(LIBC_0_2_172),
                compile(MANY_ASSOC_ITEMS),
                compile(MATCH_STRESS),
                compile(NALGEBRA_0_33_0),
                compile(NALGEBRA_0_33_0_NEW_SOLVER),
                compile(PROJECTION_CACHING),
                compile(REGEX_AUTOMATA_0_4_8),
                compile(REGRESSION_31157),
                compile(RIPGREP_14_1_1),
                compile(RIPGREP_14_1_1_TINY),
                compile(SERDE_1_0_219),
                compile(SERDE_1_0_219_NEW_SOLVER),
                compile(SERDE_1_0_219_THREADS4),
                compile(SERDE_DERIVE_1_0_219),
                compile(STM32F4_0_15_1),
                compile(SYN_2_0_101),
                compile(SYN_2_0_101_NEW_SOLVER),
                compile(TOKEN_STREAM_STRESS),
                compile(TT_MUNCHER),
                compile(TUPLE_STRESS),
                compile(TYPENUM_1_18_0),
                compile(UCD),
                compile(UNICODE_NORMALIZATION_0_1_24),
                compile(UNIFY_LINEARLY),
                compile(UNUSED_WARNINGS),
                compile(WF_PROJECTION_STRESS_65510),
                compile(WG_GRAMMAR),
            ];

            vec![BenchmarkSet { members: set }]
        }
    }
}

/// Expand all the benchmarks that should be performed by a single collector.
pub fn get_benchmark_set(id: BenchmarkSetId) -> BenchmarkSet {
    let mut sets = get_benchmark_sets_for_target(id.target);
    sets.remove(id.index as usize)
}

#[cfg(test)]
mod tests {
    use crate::benchmark_set::{get_benchmark_sets_for_target, BenchmarkSetMember};
    use crate::compile::benchmark::target::Target;
    use crate::compile::benchmark::{
        get_compile_benchmarks, BenchmarkName, CompileBenchmarkFilter,
    };
    use std::collections::HashSet;
    use std::path::Path;

    /// Sanity check for making sure that the expanded benchmark sets are non-overlapping and
    /// complete, i.e. they don't miss any benchmarks.
    #[test]
    fn check_benchmark_set_x64() {
        let sets = get_benchmark_sets_for_target(Target::X86_64UnknownLinuxGnu);

        // Assert set is unique
        for set in &sets {
            let hashset = set.members().iter().collect::<HashSet<_>>();
            assert_eq!(
                set.members().len(),
                hashset.len(),
                "Benchmark set {set:?} contains duplicates"
            );
        }

        // Go through all unique pairs of sets and assert that they don't overlap
        for i in 0..sets.len() {
            for j in i + 1..sets.len() {
                let set_a = &sets[i];
                let set_b = &sets[j];
                let hashset_a = set_a.members().iter().collect::<HashSet<_>>();
                let hashset_b = set_b.members().iter().collect::<HashSet<_>>();
                assert!(
                    hashset_a.is_disjoint(&hashset_b),
                    "Benchmark sets {set_a:?} and {set_b:?} overlap"
                );
            }
        }

        // Check that the union of all sets contains all the required benchmarks
        let all_members = sets
            .iter()
            .flat_map(|s| s.members())
            .collect::<HashSet<_>>();

        const BENCHMARK_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/compile-benchmarks");
        let all_compile_benchmarks =
            get_compile_benchmarks(Path::new(BENCHMARK_DIR), CompileBenchmarkFilter::All)
                .unwrap()
                .into_iter()
                .map(|b| b.name)
                .collect::<Vec<BenchmarkName>>();
        for benchmark in &all_compile_benchmarks {
            assert!(
                all_members.contains(&BenchmarkSetMember::CompileBenchmark(benchmark.clone())),
                "Compile-time benchmark `{benchmark}` is missing in the union of all sets"
            );
        }
        for benchmark in &all_members {
            let BenchmarkSetMember::CompileBenchmark(name) = benchmark;
            assert!(
                all_compile_benchmarks.contains(name),
                "Compile-time benchmark {name} does not exist on disk"
            );
        }
        assert_eq!(all_members.len(), all_compile_benchmarks.len());
    }
}
