//! TLS backend-specific functionality.

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod security_framework;

#[cfg(target_os = "windows")]
pub mod schannel;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
pub mod openssl;
