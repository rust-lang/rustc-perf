use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(debug_assertions)] {
        compile_error!("the verify tests only run in --release mode");
    }
}

mod math;
mod ops;
mod reductions;
