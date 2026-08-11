use std::{env, fmt, path::{Path, PathBuf}, error::Error};
use nix::NixPath;

#[derive(Debug, PartialEq)]
pub enum CliError {
	// Parsing errors.
	EmptyProgname,
	NotEnoughtArgs,
	CallHelp,
	UnknownFlag(String),
	// Validating errors.
	EmptyExecutable,
	ExecutableNotFound(which::Error),
}

impl fmt::Display for CliError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CliError::EmptyProgname => write!(f, "unable to get program name"),
			CliError::NotEnoughtArgs => write!(f, "not enought input arguments"),
			CliError::CallHelp => write!(f, "get help flag"),
			CliError::UnknownFlag(flag) => write!(f, "unknown argument: \'{}\'", flag),
			CliError::EmptyExecutable => write!(f, "empty program name"),
			CliError::ExecutableNotFound(e) => write!(f, "executable not found: {}", e),
		}
	}
}

impl Error for CliError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			CliError::EmptyProgname => None,
			CliError::NotEnoughtArgs => None,
			CliError::CallHelp => None,
			CliError::UnknownFlag(_flag) => None,
			CliError::EmptyExecutable => None,
			CliError::ExecutableNotFound(e) => Some(e),
		}
	}
}

impl From<which::Error> for CliError {
	fn from(e: which::Error) -> Self {
		CliError::ExecutableNotFound(e)
	}
}

pub struct CliArgs {
	progname: String,
	child_command: PathBuf,
	child_args: Vec<String>,
	only_summary: bool,
	follow_forks: bool,
}

impl CliArgs {

	pub fn new() -> Self {
		CliArgs {
			progname: String::new(),
			child_command: PathBuf::new(),
			child_args: Vec::new(),
			only_summary: false,
			follow_forks: false,
		}
	}

	pub fn parse(&mut self) -> Result<(), CliError> {
		
		let args: Vec<String> = env::args().collect();

		if args.is_empty() {
			return Err(CliError::EmptyProgname);
		}

		// Get 'prog' from '/path/to/my/prog' and write it to CliArgs struct.
		let progname = args.get(0)
			.map(Path::new)
			.and_then(|p| p.file_name())
			.and_then(|n| n.to_str())
			.unwrap_or("");
		self.progname.push_str(progname);

		if args.len() < 2 {
			return Err(CliError::NotEnoughtArgs);
		}

		let mut args_i = args.into_iter().skip(1);

		while let Some(arg) = args_i.next() {
			match arg.as_str() {
				"-?" | "--help" => {
					return Err(CliError::CallHelp);
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
				f @ _ => {
					return Err(CliError::UnknownFlag(f.to_string()));
				}
			}
		}

		Ok(())
	}

	pub fn validate(&mut self) -> Result<(), CliError> {

		if self.child_command.is_empty() {
			return Err(CliError::EmptyExecutable);
		}

		// Checking given command for it is exist and executable.
		match which::which(&self.child_command) {
			Ok(path) => self.child_command = path,
			Err(e) => return Err(CliError::ExecutableNotFound(e))
		}

		Ok(())
	}

	pub fn usage(&self) -> String {
		format!("Usage: {} [OPTION] <binary_name> [BINARY_ARGS]", self.progname)
	}
}