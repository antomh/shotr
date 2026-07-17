use std::{env, path::{Path, PathBuf}};
use nix::NixPath;

pub mod tracer;
pub mod decoder;
pub mod formatter;
pub mod memory;

struct ArgValue {
	progname: String,
	child_command: PathBuf,
	child_args: Vec<String>,
	only_summary: bool,
	follow_forks: bool,
}

impl ArgValue {

	fn parse(&mut self) -> Result<(), String> {
		
		let args: Vec<String> = env::args().collect();

		if args.is_empty() {
			return Err("unable to get program name".to_string());
		}

		// Get 'prog' from '/path/to/my/prog' and write it to ArgValue struct.
		let progname = args.get(0)
			.map(Path::new)
			.and_then(|p| p.file_name())
			.and_then(|n| n.to_str())
			.unwrap_or("");
		self.progname.push_str(progname);

		dbg!(&args);

		if args.len() < 2 {
			return Err(
				format!("not enought arguments\n{}", usage(&self.progname))
			);
		}

		let mut args_i = args.into_iter().skip(1);

		while let Some(arg) = args_i.next() {
			match arg.as_str() {
				"-?" | "--help" => {
					println!("This must be the help message...");
					return Ok(());
				},
				"-c" | "--summary-only" => {
					self.only_summary = true;
				},
				"-f" | "--follow-forks" => {
					self.follow_forks = true;
				},
				// First argument without '-' is a <binary_name>.
				cmd if !cmd.starts_with('-') => {
					self.child_command = PathBuf::from(arg);
					// Everything after <binary_name> are an argument to it. 
					self.child_args = args_i.collect();
					break;
				},
				_ => {
					return Err(
						format!("unknown argument: \'{}\'", arg)
					);
				}
			}
		}

		Ok(())
	}

	fn validate(&mut self) -> Result<(), String> {

		if self.child_command.is_empty() {
			return Err(
				format!("empty program name!\n{}", usage(&self.progname))
			);
		}

		// Checking given command for it is exist and executable.
		match which::which(&self.child_command) {
			Ok(path) => self.child_command = path,
			Err(e) => return Err(
				format!("[{}]: {}", self.child_command.display(), e.to_string())
			)
		}

		Ok(())
	}
}

pub fn run() -> Result<(), String> {

	let mut opts = ArgValue {
		progname: String::new(),
		child_command: PathBuf::new(),
		child_args: Vec::new(),
		only_summary: false,
		follow_forks: false
	};

	if let Err(e) = opts.parse() {
		return Err(
			format!("Error while argument parsing: {e}")
		);
	}

	if let Err(e) = opts.validate() {
		return Err(
			format!("Error while argument validating: {e}")
		);
	}

	Ok(())
}

fn usage(progname: &str) -> String {
	format!("Usage: {} [OPTION] <binary_name> [BINARY_ARGS]", progname)
}
