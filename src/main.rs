use anyhow::Result;
use clap::Parser;

/// Detect CPU capabilities and output Gentoo `CPU_FLAGS_*` values.
///
/// Reads CPU feature information (CPUID on x86, `AT_HWCAP` on ARM/PPC/RISC-V)
/// and prints the matching set of `CPU_FLAGS` for use in `/etc/portage/package.use`.
#[derive(Parser)]
#[command(name = "cpuid2cpuflags", version, about)]
struct Cli {
    // Future: --from-cflags, --from-march, --json, etc.
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    let flags = cpuid2cpuflags::detect()?;
    println!("{flags}");

    Ok(())
}
