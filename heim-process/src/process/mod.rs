use std::fmt;
use std::path::PathBuf;

use heim_common::prelude::*;

use crate::{sys, Pid, ProcessResult};

mod cpu_times;
mod memory;
mod status;

pub use self::cpu_times::CpuTime;
pub use self::memory::Memory;
pub use self::status::Status;

/// OS process.
#[derive(heim_derive::ImplWrap)]
pub struct Process(sys::Process);

impl Process {
    /// Load the process information with `pid` given.
    pub fn get(pid: Pid) -> impl Future<Output = ProcessResult<Self>> {
        sys::Process::get(pid).map_ok(Self)
    }

    /// Returns the `Process` matching the currently running program.
    pub fn current() -> impl Future<Output = ProcessResult<Self>> {
        sys::Process::current().map_ok(Self)
    }

    /// Returns the process pid.
    pub fn pid(&self) -> Pid {
        self.as_ref().pid()
    }

    /// Returns future which resolves into the process parent pid.
    pub fn parent_pid(&self) -> impl Future<Output = ProcessResult<Pid>> {
        self.as_ref().parent_pid()
    }

    /// Returns future which resolves into the process name.
    pub fn name(&self) -> impl Future<Output = ProcessResult<String>> {
        self.as_ref().name()
    }

    /// Returns future which resolves into the process executable as an absolute path.
    pub fn exe(&self) -> impl Future<Output = ProcessResult<PathBuf>> {
        self.as_ref().exe()
    }

    /// Returns future which resolves into the current process status.
    pub fn status(&self) -> impl Future<Output = ProcessResult<Status>> {
        self.as_ref().status()
    }

    /// Returns future which resolves into the accumulated process time.
    pub fn cpu_time(&self) -> impl Future<Output = ProcessResult<CpuTime>> {
        self.as_ref().cpu_time().map_ok(Into::into)
    }

    /// Returns future which resolves into the memory information about this process.
    pub fn memory(&self) -> impl Future<Output = ProcessResult<Memory>> {
        self.as_ref().memory().map_ok(Into::into)
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid()).finish()
    }
}

/// Returns stream which yields currently running processes.
pub fn processes() -> impl Stream<Item = ProcessResult<Process>> {
    sys::processes().map_ok(Into::into)
}
