//! CLI integration tests.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_flag() {
    Command::cargo_bin("cpuid2cpuflags")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("CPU_FLAGS"));
}

#[test]
fn version_flag() {
    Command::cargo_bin("cpuid2cpuflags")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[test]
fn detect_produces_x86_output() {
    Command::cargo_bin("cpuid2cpuflags")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::starts_with("CPU_FLAGS_X86:"));
}
