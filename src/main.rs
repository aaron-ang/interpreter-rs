use std::{env, fs, process::ExitCode, str::FromStr};

use interpreter_starter_rust::{cli_run, CliMode, CANNOT_OPEN_INPUT, COMMAND_LINE_USAGE};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} [tokenize|parse|evaluate|run] <filename>",
            args[0]
        );
        return ExitCode::from(COMMAND_LINE_USAGE);
    }

    let mode = CliMode::from_str(&args[1]).unwrap_or_else(|_| {
        eprintln!("Unknown command: {}", args[1]);
        std::process::exit(COMMAND_LINE_USAGE as i32);
    });

    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {filename}");
        std::process::exit(CANNOT_OPEN_INPUT as i32);
    });

    let mut out = std::io::stdout();
    let mut err = std::io::stderr();
    let exit_code = cli_run(mode, &file_contents, &mut out, &mut err);
    ExitCode::from(exit_code)
}
