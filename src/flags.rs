use std::collections::BTreeSet;
use std::fmt;

/// The target architecture for CPU flag detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arch {
    /// x86 and x86\_64 (amd64).
    X86,
    /// ARM and `AArch64`.
    Arm,
    /// PowerPC and `PowerPC64`.
    Ppc,
    /// RISC-V (requires `riscv` feature).
    #[cfg(feature = "riscv")]
    RiscV,
}

impl Arch {
    /// Returns the Gentoo `CPU_FLAGS_*` prefix for this architecture.
    #[must_use]
    pub fn prefix(self) -> &'static str {
        match self {
            Self::X86 => "CPU_FLAGS_X86",
            Self::Arm => "CPU_FLAGS_ARM",
            Self::Ppc => "CPU_FLAGS_PPC",
            #[cfg(feature = "riscv")]
            Self::RiscV => "CPU_FLAGS_RISCV",
        }
    }
}

/// A set of detected CPU flags for a specific architecture.
///
/// Flags are stored sorted and deduplicated for deterministic output.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlagSet {
    arch: Arch,
    flags: BTreeSet<String>,
}

impl FlagSet {
    /// Creates a new empty flag set for the given architecture.
    #[must_use]
    pub fn new(arch: Arch) -> Self {
        Self {
            arch,
            flags: BTreeSet::new(),
        }
    }

    /// Inserts a flag name into the set.
    pub fn insert(&mut self, flag: &str) {
        self.flags.insert(flag.to_owned());
    }

    /// Returns the architecture of this flag set.
    #[must_use]
    pub fn arch(&self) -> Arch {
        self.arch
    }

    /// Returns the flags as a sorted slice of strings.
    #[must_use]
    pub fn flags(&self) -> Vec<&str> {
        self.flags.iter().map(String::as_str).collect()
    }

    /// Returns `true` if the flag set contains the given flag.
    #[must_use]
    pub fn contains(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    /// Returns the number of flags in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.flags.len()
    }

    /// Returns `true` if the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
}

impl fmt::Display for FlagSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.arch.prefix(), self.flags().join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_sorted_output() {
        let mut fs = FlagSet::new(Arch::X86);
        fs.insert("sse2");
        fs.insert("aes");
        fs.insert("mmx");
        assert_eq!(fs.to_string(), "CPU_FLAGS_X86: aes mmx sse2");
    }

    #[test]
    fn deduplication() {
        let mut fs = FlagSet::new(Arch::Arm);
        fs.insert("neon");
        fs.insert("neon");
        assert_eq!(fs.len(), 1);
    }
}
