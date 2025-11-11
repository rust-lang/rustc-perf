//! This file contains an exhaustive list of all compile-time benchmarks
//! located in the `collector/compile-benchmarks` directory that are benchmarked in production.
//! If new benchmarks are added/removed, they have to also be added/removed here, and in
//! the [super::expand_benchmark_set] function.

// Stable benchmarks
pub(super) const CARGO: &str = "cargo";
pub(super) const ENCODING: &str = "encoding";
pub(super) const FUTURES: &str = "futures";
pub(super) const HTML5EVER: &str = "html5ever";
pub(super) const INFLATE: &str = "inflate";
pub(super) const PISTON_IMAGE: &str = "piston-image";
pub(super) const REGEX: &str = "regex";
pub(super) const SYN: &str = "syn";
pub(super) const TOKIO_WEBPUSH_SIMPLE: &str = "tokio-webpush-simple";

// Non-stable benchmarks
pub(super) const AWAIT_CALL_TREE: &str = "await-call-tree";
pub(super) const BITMAPS_3_2_1: &str = "bitmaps-3.2.1";
pub(super) const BITMAPS_3_2_1_NEW_SOLVER: &str = "bitmaps-3.2.1-new-solver";
pub(super) const CARGO_0_87_1: &str = "cargo-0.87.1";
pub(super) const CLAP_DERIVE_4_5_32: &str = "clap_derive-4.5.32";
pub(super) const COERCIONS: &str = "coercions";
pub(super) const CRANELIFT_CODEGEN_0_119_0: &str = "cranelift-codegen-0.119.0";
pub(super) const CTFE_STRESS_5: &str = "ctfe-stress-5";
pub(super) const DEEP_VECTOR: &str = "deep-vector";
pub(super) const DEEPLY_NESTED_MULTI: &str = "deeply-nested-multi";
pub(super) const DERIVE: &str = "derive";
pub(super) const DIESEL_2_2_10: &str = "diesel-2.2.10";
pub(super) const EXTERNS: &str = "externs";
pub(super) const EZA_0_21_2: &str = "eza-0.21.2";
pub(super) const HELLOWORLD: &str = "helloworld";
pub(super) const HELLOWORLD_TINY: &str = "helloworld-tiny";
pub(super) const HTML5EVER_0_31_0: &str = "html5ever-0.31.0";
pub(super) const HTML5EVER_0_31_0_NEW_SOLVER: &str = "html5ever-0.31.0-new-solver";
pub(super) const HYPER_1_6_0: &str = "hyper-1.6.0";
pub(super) const IMAGE_0_25_6: &str = "image-0.25.6";
pub(super) const INCLUDE_BLOB: &str = "include-blob";
pub(super) const ISSUE_46449: &str = "issue-46449";
pub(super) const ISSUE_58319: &str = "issue-58319";
pub(super) const ISSUE_88862: &str = "issue-88862";
pub(super) const LARGE_WORKSPACE: &str = "large-workspace";
pub(super) const LIBC_0_2_172: &str = "libc-0.2.172";
pub(super) const MANY_ASSOC_ITEMS: &str = "many-assoc-items";
pub(super) const MATCH_STRESS: &str = "match-stress";
pub(super) const NALGEBRA_0_33_0: &str = "nalgebra-0.33.0";
pub(super) const NALGEBRA_0_33_0_NEW_SOLVER: &str = "nalgebra-0.33.0-new-solver";
pub(super) const PROJECTION_CACHING: &str = "projection-caching";
pub(super) const REGEX_AUTOMATA_0_4_8: &str = "regex-automata-0.4.8";
pub(super) const REGRESSION_31157: &str = "regression-31157";
pub(super) const RIPGREP_14_1_1: &str = "ripgrep-14.1.1";
pub(super) const RIPGREP_14_1_1_TINY: &str = "ripgrep-14.1.1-tiny";
pub(super) const SERDE_1_0_219: &str = "serde-1.0.219";
pub(super) const SERDE_1_0_219_NEW_SOLVER: &str = "serde-1.0.219-new-solver";
pub(super) const SERDE_1_0_219_THREADS4: &str = "serde-1.0.219-threads4";
pub(super) const SERDE_DERIVE_1_0_219: &str = "serde_derive-1.0.219";
pub(super) const STM32F4_0_15_1: &str = "stm32f4-0.15.1";
pub(super) const SYN_2_0_101: &str = "syn-2.0.101";
pub(super) const SYN_2_0_101_NEW_SOLVER: &str = "syn-2.0.101-new-solver";
pub(super) const TOKEN_STREAM_STRESS: &str = "token-stream-stress";
pub(super) const TT_MUNCHER: &str = "tt-muncher";
pub(super) const TUPLE_STRESS: &str = "tuple-stress";
pub(super) const TYPENUM_1_18_0: &str = "typenum-1.18.0";
pub(super) const UCD: &str = "ucd";
pub(super) const UNICODE_NORMALIZATION_0_1_24: &str = "unicode-normalization-0.1.24";
pub(super) const UNIFY_LINEARLY: &str = "unify-linearly";
pub(super) const UNUSED_WARNINGS: &str = "unused-warnings";
pub(super) const WF_PROJECTION_STRESS_65510: &str = "wf-projection-stress-65510";
pub(super) const WG_GRAMMAR: &str = "wg-grammar";
