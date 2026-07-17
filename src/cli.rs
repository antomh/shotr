use std::{env, path::{Path, PathBuf}};
use nix::NixPath;

pub struct CliArgs {
	progname: String,
	child_command: PathBuf,
	child_args: Vec<String>,
	only_summary: bool,
	follow_forks: bool,
	help: bool,
}

impl CliArgs {

	pub fn new() -> Self {
		CliArgs {
			progname: String::new(),
			child_command: PathBuf::new(),
			child_args: Vec::new(),
			only_summary: false,
			follow_forks: false,
			help: false,
		}
	}

	pub fn parse(&mut self) -> Result<(), String> {
		
		let args: Vec<String> = env::args().collect();

		if args.is_empty() {
			return Err("unable to get program name".to_string());
		}

		// Get 'prog' from '/path/to/my/prog' and write it to CliArgs struct.
		let progname = args.get(0)
			.map(Path::new)
			.and_then(|p| p.file_name())
			.and_then(|n| n.to_str())
			.unwrap_or("");
		self.progname.push_str(progname);

		if args.len() < 2 {
			return Err(
				format!("not enought arguments\n{}", self.usage())
			);
		}

		let mut args_i = args.into_iter().skip(1);

		while let Some(arg) = args_i.next() {
			match arg.as_str() {
				"-?" | "--help" => {
					self.help = true;
					self.usage();
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

	pub fn validate(&mut self) -> Result<(), String> {

		if self.child_command.is_empty() {
			return Err(
				format!("empty program name!\n{}", self.usage())
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

	pub fn help(&self) -> bool {
		self.help
	}

	fn usage(&self) -> String {
		format!("Usage: {} [OPTION] <binary_name> [BINARY_ARGS]", self.progname)
	}
}