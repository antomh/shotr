use std::{collections::HashMap, fmt, path::Path, error::Error};

use nix::unistd::Pid;

#[derive(Debug, PartialEq)]
pub enum TracerError {
	ForkFailed(nix::Error),
	ExecFailed,
	PtraceFailed(nix::Error),
	WaitFailed(nix::Error),
}

impl fmt::Display for TracerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TracerError::ForkFailed(e) => write!(f, "failed to fork: {e}"),
			TracerError::ExecFailed => write!(f, "target process failed to start"),
			TracerError::PtraceFailed(e) => write!(f, "ptrace call failed: {e}"),
			TracerError::WaitFailed(e) => write!(f, "waitpid failed {e}"),
		}
	}
}

impl Error for TracerError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TracerError::ForkFailed(e) => Some(e),
			TracerError::ExecFailed => None,
			TracerError::PtraceFailed(e) => Some(e),
			TracerError::WaitFailed(e) => Some(e),
		}
	}
}

enum ThreadState {
	WaitingForEntry,
	WaitingForExit,
}

pub struct Tracer {
	children: HashMap<Pid, ThreadState>,
}

impl Tracer {
	pub fn spawn(target: &Path, args: &[String]) -> Result<Self, TracerError> {
		todo!()
	}
}