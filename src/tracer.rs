use std::{
    collections::HashMap, error::Error, ffi::CString, fmt, os::unix::ffi::OsStrExt,
    path::Path,
};

use nix::libc;
use nix::sys;
use nix::unistd;

#[derive(Debug)]
pub enum TracerError {
    ForkFailed(nix::Error),
    ExecFailed,
    ChildStopped,
    SetOptFailed(nix::Error),
    PtraceFailed(nix::Error),
    WaitFailed(nix::Error),
}

impl fmt::Display for TracerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TracerError::ForkFailed(e) => write!(f, "failed to fork: {e}"),
            TracerError::ExecFailed => write!(f, "target process failed to start"),
            TracerError::ChildStopped => write!(f, "target process was stopped before tracing"),
            TracerError::SetOptFailed(e) => write!(f, "setoptions failed: {e}"),
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
            TracerError::ChildStopped => None,
            TracerError::SetOptFailed(e) => Some(e),
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
    children: HashMap<unistd::Pid, ThreadState>,
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
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();
        
        match unsafe { unistd::fork() }.map_err(|e| TracerError::ForkFailed(e))? {
            unistd::ForkResult::Child => Self::exec_child(&target, &args),
            unistd::ForkResult::Parent { child } => Self::init_parent(child),
        }
    }

    fn exec_child(target: &CString, args: &[CString]) -> ! {
        if sys::ptrace::traceme().is_err() {
            unsafe { libc::_exit(127) };
        }
        
        let mut args0 = Vec::with_capacity(1 + args.len());
        args0.push(target.as_c_str());
        args0.extend(args.iter().map(|a| a.as_c_str()));

        match unistd::execvp(target, &args0) {
            Err(_) => unsafe { libc::_exit(127) },
            Ok(never) => match never {},
        }
    }

    fn init_parent(child: unistd::Pid) -> Result<Self, TracerError> {
        let mut tracer = Tracer::new();
        match sys::wait::waitpid(child, None) {
            Err(e) => return Err(TracerError::WaitFailed(e)),
            Ok(r) => {
                match r {
                    sys::wait::WaitStatus::Stopped(p, sys::signal::Signal::SIGTRAP) => {
                        tracer.children.insert(p, ThreadState::WaitingForEntry);
                        if let Err(e) = sys::ptrace::setoptions(p, sys::ptrace::Options::PTRACE_O_TRACESYSGOOD) {
                            return Err(TracerError::SetOptFailed(e));
                        }
                    },
                    sys::wait::WaitStatus::Exited(_, _) |
                    sys::wait::WaitStatus::Signaled(_, _, _) => return Err(TracerError::ExecFailed),
                    _ => return Err(TracerError::ChildStopped),
                }
            }
        };
        Ok(tracer)
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
    Entry { pid: unistd::Pid, syscall_num: u64, raw_args: [u64; 6] },
    Exit  { pid: unistd::Pid, retval: i64 },
    ProcessExited { pid: unistd::Pid, code: i32 },
}
