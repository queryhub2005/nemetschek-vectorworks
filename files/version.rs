//! Static build info for Nemetschek Vectorworks.

pub const BUILD_VERSION: &str  = "v1.7.7";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "404d9dba0007";
pub const BUILD_TIMESTAMP: u64 = 1700985627;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Nemetschek Vectorworks", BUILD_VERSION, BUILD_CHANNEL)
}
