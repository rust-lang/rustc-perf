pub fn profile_function<F: Fn() -> Bench, R, Bench: FnOnce() -> R>(benchmark_constructor: &F) {
    let func = benchmark_constructor();

    // With the `precise-cachegrind` feature, we want to enable cachegrind recording
    // only for the actual execution of the profiled function.
    #[cfg(feature = "precise-cachegrind")]
    {
        crabgrind::cachegrind::start_instrumentation();
    }

    func();

    #[cfg(feature = "precise-cachegrind")]
    {
        crabgrind::cachegrind::stop_instrumentation();
    }
}
