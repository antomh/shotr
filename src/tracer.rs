use std::{
    collections::HashMap, error::Error, ffi::CString, fmt, os::unix::ffi::OsStrExt,
    path::Path,
};

use nix::unistd::{self, ForkResult, Pid};
use nix::sys::ptrace;

#[derive(Debug)]
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
    fn new() -> Self {
        Tracer {
            children: HashMap::new(),
        }
    }

    pub fn spawn(target: &Path, args: &[String]) -> Result<Self, TracerError> {
        // &Path not implement From<&Path> for Vec<u8> needed for new().
        let target = target.as_os_str().as_bytes();
        let target = CString::new(target).unwrap();
        let args: Vec<CString> = args
            .into_iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();
        
        match unsafe { unistd::fork() }.map_err(|e| TracerError::ForkFailed(e))? {
            ForkResult::Child => Self::exec_child(&target, &args),
            ForkResult::Parent { child } => Self::init_parent(child),
        }
    }

    fn exec_child(target: &CString, args: &[CString]) -> ! {
        todo!()
    }

    fn init_parent(child: Pid) -> Result<Self, TracerError> {
        todo!()
    }
}

impl Iterator for Tracer {
    type Item = Result<RawSyscallEvent, TracerError>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[rustfmt::skip]
pub enum RawSyscallEvent {
    Entry { pid: Pid, syscall_num: u64, raw_args: [u64; 6] },
    Exit  { pid: Pid, retval: i64 },
    ProcessExited { pid: Pid, code: i32 },
}
