//! Process RSS sampler (sysinfo-based).

use sysinfo::{Pid, ProcessesToUpdate, System};

#[inline]
fn bytes_to_mib(bytes: u128) -> u64 {
    (bytes / (1024u128 * 1024)) as u64
}

pub struct MemSampler {
    sys: System,
    pid: Pid,
    peak_mib: u64,
}

impl MemSampler {
    pub fn new_current() -> Self {
        let mut s = Self {
            sys: System::new(),
            pid: Pid::from_u32(std::process::id()),
            peak_mib: 0,
        };
        s.peak_mib = s.current_mib();
        s
    }

    /// Refresh only this process and return RSS in MiB.
    pub fn current_mib(&mut self) -> u64 {
        self.sys
            .refresh_processes(ProcessesToUpdate::Some(&[self.pid]), true);
        if let Some(p) = self.sys.process(self.pid) {
            bytes_to_mib(p.memory() as u128) // sysinfo 0.37.x returns bytes
        } else {
            0
        }
    }

    /// Conditional sampling during insert loop.
    pub fn maybe_sample(&mut self, i: usize, step: usize) {
        if i.is_multiple_of(step) {
            self.update_peak();
        }
    }

    /// Update the peak based on current reading.
    pub fn update_peak(&mut self) {
        let now = self.current_mib();
        if now > self.peak_mib {
            self.peak_mib = now;
        }
    }

    /// Peak observed so far.
    pub fn peak(&self) -> u64 {
        self.peak_mib
    }
}
