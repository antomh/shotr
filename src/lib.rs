use std::fmt::Display;

use crate::tracer::Tracer;

pub mod cli;
pub mod decoder;
pub mod formatter;
pub mod memory;
pub mod tracer;

#[derive(Debug)]
pub enum AppError {
    Parcer(cli::CliError),
    Validator(cli::CliError),
    Tracer(tracer::TracerError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Parcer(e) => write!(f, "parser: {e}"),
            AppError::Validator(e) => write!(f, "validator: {e}"),
            AppError::Tracer(e) => write!(f, "tracer: {e}"),
        }
    }
}

impl From<tracer::TracerError> for AppError {
    fn from(value: tracer::TracerError) -> Self {
        AppError::Tracer(value)
    }
}

pub fn run() -> Result<(), AppError> {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = cli::CliArgs::parse(args).map_err(|e| AppError::Parcer(e))?;

    // Show help and do nothing more if we have flag.
    if opts.show_help == true {
        println!("{}", opts.usage());
        return Ok(());
    }

    opts.validate().map_err(|e| AppError::Validator(e))?;

    let _tracer = Tracer::spawn(&opts.child_command, &opts.child_args)?;

    Ok(())
}
