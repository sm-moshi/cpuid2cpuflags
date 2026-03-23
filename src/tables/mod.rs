//! CPU flag definition tables.
//!
//! Tables are always compiled on all platforms (no `#[cfg]` gating) to enable
//! cross-platform mock testing and CFLAGS conversion on any host.

pub mod x86;

/// Which CPU register or data source to check for a flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckSource {
    // x86: CPUID leaf 0x00000001
    /// ECX register of CPUID leaf 0x00000001.
    IntelEcx,
    /// EDX register of CPUID leaf 0x00000001.
    IntelEdx,

    // x86: CPUID leaf 0x00000007, sub-leaf 0
    /// EBX register of CPUID leaf 0x00000007 / sub-leaf 0.
    IntelSub0Ebx,
    /// ECX register of CPUID leaf 0x00000007 / sub-leaf 0.
    IntelSub0Ecx,
    /// EDX register of CPUID leaf 0x00000007 / sub-leaf 0.
    IntelSub0Edx,

    // x86: CPUID leaf 0x00000007, sub-leaf 1
    /// EAX register of CPUID leaf 0x00000007 / sub-leaf 1.
    IntelSub1Eax,

    // x86: CPUID leaf 0x80000001 (AMD extended)
    /// ECX register of CPUID leaf 0x80000001.
    AmdEcx,
    /// EDX register of CPUID leaf 0x80000001.
    AmdEdx,

    // x86: CPUID leaf 0xC0000001 (Centaur/VIA)
    /// EDX register of CPUID leaf 0xC0000001.
    ViaEdx,
}

/// A single flag definition in a detection table.
#[derive(Debug, Clone, Copy)]
pub struct FlagDef {
    /// The Gentoo `CPU_FLAGS` name (e.g. "aes", "avx512f").
    pub name: &'static str,
    /// Which register/source to check.
    pub source: CheckSource,
    /// Bitmask to test against the register value.
    pub mask: u32,
}
