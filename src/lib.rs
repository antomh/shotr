pub mod cli;
pub mod tracer;
pub mod decoder;
pub mod formatter;
pub mod memory;

pub fn run() -> Result<(), String> {

	let mut opts = cli::CliArgs::new();

	if let Err(e) = opts.parse() {
		return Err(
			format!("Error while argument parsing: {e}")
		);
	} else {
		if opts.help() {
			return Ok(());
		}
	}

	if let Err(e) = opts.validate() {
		return Err(
			format!("Error while argument validating: {e}")
		);
	}

	Ok(())
}
