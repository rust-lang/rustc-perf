#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::benchmark_function;

#[cfg(windows)]
pub use windows::benchmark_function;
