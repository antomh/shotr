use std::{collections::HashMap, path::Path};

use nix::unistd::Pid;

pub enum TracerError {}

enum ThreadState {
	WaitingForEntry,
	WaitingForExit
}

pub struct Tracer {
	children: HashMap<Pid, ThreadState>
}

impl Tracer {
	pub fn spawn(target: &Path, args: &[String]) -> Result<Self, TracerError> {
		todo!()
	}
}