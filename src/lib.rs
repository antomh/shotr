pub mod cli;
pub mod tracer;
pub mod decoder;
pub mod formatter;
pub mod memory;

pub fn run() -> Result<(), String> {

	let mut opts = cli::CliArgs::new();

	if let Err(e) = opts.parse() {
		match e {
			cli::CliError::CallHelp => {
				println!("{}", opts.usage());
				return Ok(());
			},
			_ => return Err(format!("Error while argument parsing: {e}")),
		}
	}

	if let Err(e) = opts.validate() {
		if e == cli::CliError::EmptyExecutable {
				println!("{}", opts.usage());
		};
		return Err(format!("Error while argument validating: {e}"));
	}

	Ok(())
}
