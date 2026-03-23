//! Platform-specific CPU detection backends.
//!
//! Only the detection module for the current target architecture is compiled.
//! The mock module is always available for testing.

pub mod mock;

// x86 module is always compiled — tables + mock parsing need it on all platforms.
// Only the native `read_cpuid()` function is cfg-gated inside.
pub mod x86;

use crate::error::Error;
use crate::flags::FlagSet;

/// Detect CPU flags for the current platform.
///
/// Returns a [`FlagSet`] containing all detected Gentoo `CPU_FLAGS`.
///
/// # Errors
///
/// Returns [`Error::UnsupportedArch`] if the current architecture is not supported.
pub fn detect() -> Result<FlagSet, Error> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        return x86::detect();
    }

    #[allow(unreachable_code)]
    Err(Error::UnsupportedArch {
        arch: std::env::consts::ARCH,
        os: std::env::consts::OS,
    })
}
