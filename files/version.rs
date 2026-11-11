//! Static build info for Antares Auto Key 2.

pub const BUILD_VERSION: &str  = "v3.6.0";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "36ba00573061";
pub const BUILD_TIMESTAMP: u64 = 1735345598;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Antares Auto Key 2", BUILD_VERSION, BUILD_CHANNEL)
}
