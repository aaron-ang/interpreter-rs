mod callable;
mod cli;
mod constants;
mod environment;
mod error;
mod grammar;
mod interpreter;
mod parser;
mod resolver;
mod scanner;

pub use cli::{run as cli_run, Mode as CliMode};
pub use constants::exit_codes::*;
pub use grammar::Literal;
pub use interpreter::Interpreter;
pub use parser::Parser;
pub use resolver::Resolver;
pub use scanner::Scanner;

pub mod test_support {
    use std::fs;

    use crate::{cli_run, CliMode};

    /// # Panics
    /// Panics if the test file cannot be read or if the output does not match expectations.
    pub fn run_lox_test(path: &str) {
        let source = fs::read_to_string(path).expect("read test file");
        let mode = if path.contains("/scanning/") {
            CliMode::Tokenize
        } else if path.contains("/expressions/") {
            if path.ends_with("/parse.lox") {
                CliMode::Parse
            } else {
                CliMode::Evaluate
            }
        } else {
            CliMode::Run
        };

        let mut out = Vec::new();
        let mut err = Vec::new();
        let _ = cli_run(mode, &source, &mut out, &mut err);

        // Validate expectations from comments.
        let mut expected_out = Vec::new();
        let mut expected_err = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim_start();
            if let Some(comment_start) = trimmed.find("//") {
                let comment = &trimmed[comment_start..];
                if let Some(rest) = comment.strip_prefix("// expect: ") {
                    expected_out.push(rest.to_string());
                    continue;
                }
                if let Some(rest) = comment.strip_prefix("// expect runtime error: ") {
                    expected_err.push(rest.to_string());
                    continue;
                }
                if let Some(rest) = comment.strip_prefix("// ") {
                    if rest.starts_with("[line ") || rest.starts_with("Error") {
                        expected_err.push(rest.to_string());
                    }
                }
            }
        }

        let out_s = String::from_utf8_lossy(&out);
        let err_s = String::from_utf8_lossy(&err);
        let actual_out = out_s.lines().collect::<Vec<_>>();
        let actual_err = err_s
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !(trimmed.starts_with("[line ") && trimmed.ends_with(']'))
            })
            .collect::<Vec<_>>();

        assert_eq!(
            actual_out.len(),
            expected_out.len(),
            "stdout lines < expected.\nstdout:\n{out_s}"
        );
        assert_eq!(
            actual_err.len(),
            expected_err.len(),
            "stderr lines < expected.\nstderr:\n{err_s}"
        );

        for (expected, actual) in expected_out.iter().zip(actual_out) {
            assert_eq!(
                expected, actual,
                "stdout mismatch.\nExpected:\n{expected}\nActual:\n{actual}"
            );
        }
        for (expected, actual) in expected_err.iter().zip(actual_err) {
            if expected.starts_with("Error") {
                assert!(
                    actual.contains(expected),
                    "stderr missing '{expected}'.\nActual:\n{actual}"
                );
            } else {
                let expected_prefix = expected.trim_end_matches('.');
                assert!(
                    actual.starts_with(expected_prefix),
                    "stderr mismatch.\nExpected prefix: '{expected_prefix}'\nActual: '{actual}'"
                );
            }
        }
    }
}

#[cfg(test)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/lox_generated_tests.rs"));
}
