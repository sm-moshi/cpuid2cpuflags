//! Upstream compatibility tests.
//!
//! These load the verbatim test data files from `gentoo/cpuid2cpuflags` v17
//! and verify that our Rust implementation produces byte-identical output.
//! DO NOT modify the fixture files — they are the compatibility contract.

use std::path::Path;

use cpuid2cpuflags::detect::mock;

fn assert_fixture(path: &str) {
    let path = Path::new(path);
    let (expected, detected) = mock::run_fixture(path).unwrap_or_else(|e| {
        panic!("failed to run fixture {}: {e}", path.display());
    });

    let detected_str = detected.flags().join(" ");
    assert_eq!(
        expected, detected_str,
        "output mismatch for fixture {}",
        path.display()
    );
}

// --- x86 fixtures ---

#[test]
fn x86_amd_colfax() {
    assert_fixture("tests/fixtures/x86/amd-colfax.txt");
}

#[test]
fn x86_amd_zen2() {
    assert_fixture("tests/fixtures/x86/amd-zen2.txt");
}

#[test]
fn x86_amd_zen4() {
    assert_fixture("tests/fixtures/x86/amd-zen4.txt");
}

#[test]
fn x86_amd_zen4_ryzen_7_pro_7840u() {
    assert_fixture("tests/fixtures/x86/amd-zen4-ryzen-7-pro-7840u.txt");
}

#[test]
fn x86_athlon64_windsor() {
    assert_fixture("tests/fixtures/x86/athlon64-windsor.txt");
}

#[test]
fn x86_i3_ivybridge() {
    assert_fixture("tests/fixtures/x86/i3-ivybridge.txt");
}

#[test]
fn x86_opteron_6272() {
    assert_fixture("tests/fixtures/x86/opteron-6272.txt");
}

#[test]
fn x86_xeon_e_2176g() {
    assert_fixture("tests/fixtures/x86/xeon-e-2176g.txt");
}

#[test]
fn x86_xeon_platinum_8480plus() {
    assert_fixture("tests/fixtures/x86/xeon-platinum-8480plus.txt");
}

#[test]
fn x86_xeon_silver_4410() {
    assert_fixture("tests/fixtures/x86/xeon-silver-4410.txt");
}
