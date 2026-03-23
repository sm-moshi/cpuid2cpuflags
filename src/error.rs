/// Errors that can occur during CPU flag detection.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The current architecture is not supported.
    #[error("unsupported platform: {os}/{arch} (supported: x86/x86_64 on Linux)")]
    UnsupportedArch {
        /// The CPU architecture (e.g. "aarch64", "x86\_64").
        arch: &'static str,
        /// The operating system (e.g. "linux", "macos").
        os: &'static str,
    },

    /// Failed to read CPU capabilities from the system.
    #[error("failed to read CPU capabilities: {0}")]
    DetectionFailed(String),

    /// Failed to parse mock test data (testing only).
    #[error("failed to parse mock data: {0}")]
    MockParseFailed(String),

    /// Failed to parse CFLAGS input.
    #[cfg(feature = "cflags")]
    #[error("failed to parse CFLAGS: {0}")]
    CflagsParseFailed(String),
}
