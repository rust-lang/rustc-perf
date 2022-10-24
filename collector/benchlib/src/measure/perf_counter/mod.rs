#[cfg(target_os = "linux")]
mod linux;

#[cfg(not(target_os = "linux"))]
mod non_linux;

#[cfg(target_os = "linux")]
pub use linux::benchmark_function;

#[cfg(not(target_os = "linux"))]
pub use non_linux::benchmark_function;
