//! Detect CPU capabilities and output Gentoo `CPU_FLAGS_*` values.
//!
//! This crate provides both a library API and a command-line tool for detecting
//! CPU instruction set extensions and outputting them in the format used by
//! Gentoo's `CPU_FLAGS_*` `USE_EXPAND` variables.
//!
//! # Supported architectures
//!
//! - **x86 / x86\_64**: CPUID-based detection (48 flags)
//! - **ARM / `AArch64`**: `AT_HWCAP`-based detection (planned)
//! - **PowerPC**: `AT_HWCAP`-based detection (planned)
//! - **RISC-V**: `AT_HWCAP` + ISA string detection (planned, behind `riscv` feature)
//!
//! # Example
//!
//! ```no_run
//! let flags = cpuid2cpuflags::detect().unwrap();
//! println!("{flags}");
//! // Output: CPU_FLAGS_X86: aes avx avx2 ...
//! ```

pub mod detect;
pub mod error;
pub mod flags;
pub mod tables;

pub use detect::detect;
pub use error::Error;
pub use flags::{Arch, FlagSet};
