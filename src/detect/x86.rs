//! x86 / x86\_64 CPU flag detection via CPUID.

use crate::flags::{Arch, FlagSet};
use crate::tables::CheckSource;
use crate::tables::x86::FLAGS;

/// CPUID register values collected from the CPU.
#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct CpuidRegs {
    intel_ecx: u32,
    intel_edx: u32,
    intel_sub0_ebx: u32,
    intel_sub0_ecx: u32,
    intel_sub0_edx: u32,
    intel_sub1_eax: u32,
    amd_ecx: u32,
    amd_edx: u32,
    via_edx: u32,

    got_intel: bool,
    got_intel_sub0: bool,
    got_intel_sub1: bool,
    got_amd: bool,
    got_via: bool,
}

/// Detect x86 CPU flags using the CPUID instruction.
///
/// Only available on `x86`/`x86_64` targets.
///
/// # Errors
///
/// Returns an error if CPUID data cannot be read.
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn detect() -> Result<FlagSet, crate::error::Error> {
    let regs = read_cpuid();
    Ok(check_flags(&regs))
}

/// Check flags against collected CPUID register values.
///
/// This is also used by the mock backend with pre-loaded register data.
pub(crate) fn check_flags(regs: &CpuidRegs) -> FlagSet {
    let mut flags = FlagSet::new(Arch::X86);

    for def in FLAGS {
        let reg_value = match def.source {
            CheckSource::IntelEcx if regs.got_intel => Some(regs.intel_ecx),
            CheckSource::IntelEdx if regs.got_intel => Some(regs.intel_edx),
            CheckSource::IntelSub0Ebx if regs.got_intel_sub0 => Some(regs.intel_sub0_ebx),
            CheckSource::IntelSub0Ecx if regs.got_intel_sub0 => Some(regs.intel_sub0_ecx),
            CheckSource::IntelSub0Edx if regs.got_intel_sub0 => Some(regs.intel_sub0_edx),
            CheckSource::IntelSub1Eax if regs.got_intel_sub1 => Some(regs.intel_sub1_eax),
            CheckSource::AmdEcx if regs.got_amd => Some(regs.amd_ecx),
            CheckSource::AmdEdx if regs.got_amd => Some(regs.amd_edx),
            CheckSource::ViaEdx if regs.got_via => Some(regs.via_edx),
            _ => None,
        };

        if let Some(val) = reg_value {
            if (val & def.mask) == def.mask {
                flags.insert(def.name);
            }
        }
    }

    flags
}

/// Read CPUID registers from the actual CPU.
///
/// Uses the stable `core::arch::x86_64::__cpuid` / `__cpuid_count` intrinsics
/// which wrap the CPUID instruction. This mirrors the C version's approach of
/// calling GCC's `__cpuid()` / `__cpuid_count()` builtins.
///
/// These intrinsics are safe in recent Rust — no `unsafe` needed.
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn read_cpuid() -> CpuidRegs {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::{__cpuid, __cpuid_count, __get_cpuid_max};
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{__cpuid, __cpuid_count, __get_cpuid_max};

    let mut regs = CpuidRegs::default();

    // Check max supported leaf per class
    let (max_standard, _) = __get_cpuid_max(0x0000_0000);
    let (max_extended, _) = __get_cpuid_max(0x8000_0000);
    let (max_centaur, _) = __get_cpuid_max(0xC000_0000);

    // Leaf 0x00000001: standard feature flags
    if max_standard >= 0x0000_0001 {
        let r = __cpuid(0x0000_0001);
        regs.intel_ecx = r.ecx;
        regs.intel_edx = r.edx;
        regs.got_intel = true;
    }

    // Leaf 0x00000007, sub-leaf 0: extended feature flags
    if max_standard >= 0x0000_0007 {
        let r = __cpuid_count(0x0000_0007, 0);
        regs.intel_sub0_ebx = r.ebx;
        regs.intel_sub0_ecx = r.ecx;
        regs.intel_sub0_edx = r.edx;
        regs.got_intel_sub0 = true;

        // Sub-leaf 1
        let r1 = __cpuid_count(0x0000_0007, 1);
        regs.intel_sub1_eax = r1.eax;
        regs.got_intel_sub1 = true;
    }

    // Leaf 0x80000001: AMD extended features
    if max_extended >= 0x8000_0001 {
        let r = __cpuid(0x8000_0001);
        regs.amd_ecx = r.ecx;
        regs.amd_edx = r.edx;
        regs.got_amd = true;
    }

    // Leaf 0xC0000001: VIA/Centaur extended features (padlock)
    if max_centaur >= 0xC000_0001 {
        let r = __cpuid(0xC000_0001);
        regs.via_edx = r.edx;
        regs.got_via = true;
    }

    regs
}

/// Parse CPUID register data from a mock test data file.
///
/// The format matches upstream `cpuid2cpuflags` test files:
/// - `top:LEAF:EAX:EBX:ECX:EDX` — results of `cpuid(leaf)`
/// - `sub:LEAF:SUBLEAF:EAX:EBX:ECX:EDX` — results of `cpuid_count(leaf, subleaf)`
pub(crate) fn parse_mock_x86(lines: &[&str]) -> Result<CpuidRegs, crate::error::Error> {
    let mut regs = CpuidRegs::default();

    for line in lines {
        let line = line.trim();
        if line.starts_with("expected:") || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "top" => {
                if parts.len() < 6 {
                    continue;
                }
                let leaf = u32::from_str_radix(parts[1], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let _eax = u32::from_str_radix(parts[2], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let _ebx = u32::from_str_radix(parts[3], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let ecx = u32::from_str_radix(parts[4], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let edx = u32::from_str_radix(parts[5], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;

                match leaf {
                    0x0000_0001 => {
                        regs.intel_ecx = ecx;
                        regs.intel_edx = edx;
                        regs.got_intel = true;
                    },
                    0x8000_0001 => {
                        regs.amd_ecx = ecx;
                        regs.amd_edx = edx;
                        regs.got_amd = true;
                    },
                    0xC000_0001 => {
                        regs.via_edx = edx;
                        regs.got_via = true;
                    },
                    _ => {},
                }
            },
            "sub" => {
                if parts.len() < 7 {
                    continue;
                }
                let leaf = u32::from_str_radix(parts[1], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let subleaf = u32::from_str_radix(parts[2], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let eax = u32::from_str_radix(parts[3], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let ebx = u32::from_str_radix(parts[4], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let ecx = u32::from_str_radix(parts[5], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;
                let edx = u32::from_str_radix(parts[6], 16)
                    .map_err(|e| crate::error::Error::MockParseFailed(e.to_string()))?;

                if leaf == 0x0000_0007 {
                    match subleaf {
                        0 => {
                            regs.intel_sub0_ebx = ebx;
                            regs.intel_sub0_ecx = ecx;
                            regs.intel_sub0_edx = edx;
                            regs.got_intel_sub0 = true;
                        },
                        1 => {
                            regs.intel_sub1_eax = eax;
                            regs.got_intel_sub1 = true;
                        },
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    Ok(regs)
}
