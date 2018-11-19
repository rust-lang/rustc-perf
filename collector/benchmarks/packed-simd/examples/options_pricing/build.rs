fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "ispc")]
    {
        if std::env::var("CARGO_FEATURE_ISPC").is_ok() {
            let mut cfg = ispc::Config::new();

            if cfg!(windows) {
                cfg.debug(false);
            }

            let ispc_files = vec!["volta/options.ispc"];

            for s in &ispc_files[..] {
                cfg.file(*s);
            }

            cfg.target_isas(vec![
                ispc::opt::TargetISA::SSE2i32x4,
                ispc::opt::TargetISA::SSE4i32x4,
                ispc::opt::TargetISA::AVX1i32x8,
                ispc::opt::TargetISA::AVX2i32x8,
                ispc::opt::TargetISA::AVX512KNLi32x16,
            ]);

            #[cfg(feature = "ispc_libm")]
            {
                // Use the system's libm
                cfg.math_lib(ispc::opt::MathLib::System);
            }

            cfg.compile("options");
        }
    }
}
