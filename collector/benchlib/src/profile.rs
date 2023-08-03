pub fn profile_function<F: Fn() -> Bench, R, Bench: FnOnce() -> R>(benchmark_constructor: &F) {
    let func = benchmark_constructor();
    func();
}
