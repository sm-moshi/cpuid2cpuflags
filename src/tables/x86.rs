//! x86 / x86\_64 CPU flag definitions.
//!
//! Ported verbatim from upstream `gentoo/cpuid2cpuflags` v17 `src/x86.c`.
//! Flag names and bitmasks MUST match Gentoo `profiles/desc/cpu_flags_x86.desc`.

use super::{CheckSource, FlagDef};

/// x86 CPU flag table.
///
/// Some flags appear twice (e.g. `mmxext`, `popcnt`) because they can be
/// detected via different CPUID leaves (Intel vs AMD). The detection logic
/// deduplicates by flag name.
pub const FLAGS: &[FlagDef] = &[
    FlagDef {
        name: "3dnow",
        source: CheckSource::AmdEdx,
        mask: 1 << 31,
    },
    FlagDef {
        name: "3dnowext",
        source: CheckSource::AmdEdx,
        mask: 1 << 30,
    },
    FlagDef {
        name: "aes",
        source: CheckSource::IntelEcx,
        mask: 1 << 25,
    },
    FlagDef {
        name: "amx_bf16",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 22,
    },
    FlagDef {
        name: "amx_int8",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 25,
    },
    FlagDef {
        name: "amx_tile",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 24,
    },
    FlagDef {
        name: "avx",
        source: CheckSource::IntelEcx,
        mask: 1 << 28,
    },
    FlagDef {
        name: "avx2",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 5,
    },
    FlagDef {
        name: "avx512_4fmaps",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 3,
    },
    FlagDef {
        name: "avx512_4vnniw",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 2,
    },
    FlagDef {
        name: "avx512_bf16",
        source: CheckSource::IntelSub1Eax,
        mask: 1 << 5,
    },
    FlagDef {
        name: "avx512_bitalg",
        source: CheckSource::IntelSub0Ecx,
        mask: 1 << 12,
    },
    FlagDef {
        name: "avx512_fp16",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 23,
    },
    FlagDef {
        name: "avx512_vbmi2",
        source: CheckSource::IntelSub0Ecx,
        mask: 1 << 6,
    },
    FlagDef {
        name: "avx512_vnni",
        source: CheckSource::IntelSub0Ecx,
        mask: 1 << 11,
    },
    FlagDef {
        name: "avx512_vp2intersect",
        source: CheckSource::IntelSub0Edx,
        mask: 1 << 8,
    },
    FlagDef {
        name: "avx512_vpopcntdq",
        source: CheckSource::IntelSub0Ecx,
        mask: 1 << 14,
    },
    FlagDef {
        name: "avx512bw",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 30,
    },
    FlagDef {
        name: "avx512cd",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 28,
    },
    FlagDef {
        name: "avx512dq",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 17,
    },
    FlagDef {
        name: "avx512er",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 27,
    },
    FlagDef {
        name: "avx512f",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 16,
    },
    FlagDef {
        name: "avx512ifma",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 21,
    },
    FlagDef {
        name: "avx512pf",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 26,
    },
    FlagDef {
        name: "avx512vbmi",
        source: CheckSource::IntelSub0Ecx,
        mask: 1 << 1,
    },
    FlagDef {
        name: "avx512vl",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 31,
    },
    FlagDef {
        name: "avx_vnni",
        source: CheckSource::IntelSub1Eax,
        mask: 1 << 4,
    },
    FlagDef {
        name: "bmi1",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 3,
    },
    FlagDef {
        name: "bmi2",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 8,
    },
    FlagDef {
        name: "f16c",
        source: CheckSource::IntelEcx,
        mask: 1 << 29,
    },
    FlagDef {
        name: "fma3",
        source: CheckSource::IntelEcx,
        mask: 1 << 12,
    },
    FlagDef {
        name: "fma4",
        source: CheckSource::AmdEcx,
        mask: 1 << 16,
    },
    FlagDef {
        name: "mmx",
        source: CheckSource::IntelEdx,
        mask: 1 << 23,
    },
    // mmxext: detected via AMD EDX *or* implied by SSE (Intel EDX bit 25)
    FlagDef {
        name: "mmxext",
        source: CheckSource::AmdEdx,
        mask: 1 << 22,
    },
    FlagDef {
        name: "mmxext",
        source: CheckSource::IntelEdx,
        mask: 1 << 25,
    },
    FlagDef {
        name: "padlock",
        source: CheckSource::ViaEdx,
        mask: 1 << 10,
    },
    FlagDef {
        name: "pclmul",
        source: CheckSource::IntelEcx,
        mask: 1 << 1,
    },
    // popcnt: ABM on AMD (ECX bit 5) *or* Intel (ECX bit 23)
    FlagDef {
        name: "popcnt",
        source: CheckSource::AmdEcx,
        mask: 1 << 5,
    },
    FlagDef {
        name: "popcnt",
        source: CheckSource::IntelEcx,
        mask: 1 << 23,
    },
    FlagDef {
        name: "rdrand",
        source: CheckSource::IntelEcx,
        mask: 1 << 30,
    },
    FlagDef {
        name: "sha",
        source: CheckSource::IntelSub0Ebx,
        mask: 1 << 29,
    },
    FlagDef {
        name: "sse",
        source: CheckSource::IntelEdx,
        mask: 1 << 25,
    },
    FlagDef {
        name: "sse2",
        source: CheckSource::IntelEdx,
        mask: 1 << 26,
    },
    FlagDef {
        name: "sse3",
        source: CheckSource::IntelEcx,
        mask: 1 << 0,
    },
    FlagDef {
        name: "sse4_1",
        source: CheckSource::IntelEcx,
        mask: 1 << 19,
    },
    FlagDef {
        name: "sse4_2",
        source: CheckSource::IntelEcx,
        mask: 1 << 20,
    },
    FlagDef {
        name: "sse4a",
        source: CheckSource::AmdEcx,
        mask: 1 << 6,
    },
    FlagDef {
        name: "ssse3",
        source: CheckSource::IntelEcx,
        mask: 1 << 9,
    },
    FlagDef {
        name: "vpclmulqdq",
        source: CheckSource::IntelSub0Ecx,
        mask: 1 << 10,
    },
    FlagDef {
        name: "xop",
        source: CheckSource::AmdEcx,
        mask: 1 << 11,
    },
];
