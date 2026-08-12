pub mod cli;
pub mod tracer;
pub mod decoder;
pub mod formatter;
pub mod memory;

pub fn run() -> Result<(), String> {

	let args: Vec<String> = std::env::args().collect();
	let mut opts = match cli::CliArgs::parse(args) {
		Ok(o) => {
			if o.show_help == true {
				println!("{}", o.usage());
				return Ok(());
			}
			o
		},
		Err(e) => {
			return Err(format!("parsing error: {e}"));
		}
	};

	if let Err(e) = opts.validate() {
		if e == cli::CliError::EmptyExecutable {
				println!("{}", opts.usage());
		};
		return Err(format!("argument validating error: {e}"));
	}

	Ok(())
}
