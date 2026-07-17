use std::process::ExitCode;

use shotr::run;

fn main() -> ExitCode {

    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
