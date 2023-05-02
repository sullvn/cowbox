#[cfg(target_os = "linux")]
pub const INJECTION_ENV_KEY: &str = "LD_PRELOAD";

#[cfg(target_os = "macos")]
pub const INJECTION_ENV_KEY: &str = "DYLD_INSERT_LIBRARIES";
