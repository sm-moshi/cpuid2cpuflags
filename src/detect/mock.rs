//! Mock detection backend for testing.
//!
//! Parses upstream `cpuid2cpuflags` test data files and runs the same
//! flag-checking logic against the parsed register values. This enables
//! cross-platform testing without actual hardware.

use std::path::Path;

use crate::error::Error;
use crate::flags::FlagSet;

/// Parse a mock test data file and return the expected output and detected flags.
///
/// Returns `(expected_flags_string, detected_flag_set)`.
///
/// # Errors
///
/// Returns [`Error::MockParseFailed`] if the file cannot be read or parsed.
pub fn run_fixture(path: &Path) -> Result<(String, FlagSet), Error> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::MockParseFailed(format!("failed to read {}: {e}", path.display())))?;

    let lines: Vec<&str> = content.lines().collect();

    // Extract expected output
    let expected = lines
        .iter()
        .find(|l| l.starts_with("expected:"))
        .map(|l| l.strip_prefix("expected:").unwrap_or("").to_owned())
        .ok_or_else(|| Error::MockParseFailed("no expected: line found".into()))?;

    // Determine architecture from file path
    let path_str = path.to_string_lossy();
    let detected = if path_str.contains("/x86/") {
        let regs = super::x86::parse_mock_x86(&lines)?;
        super::x86::check_flags(&regs)
    } else {
        return Err(Error::MockParseFailed(format!(
            "cannot determine architecture from path: {}",
            path.display()
        )));
    };

    Ok((expected, detected))
}
