use sys_info::mem_info;
use std::fmt;

#[derive(Debug)]
pub struct MemoryStats {
    percent: f64,
}

impl MemoryStats {
    fn new(percent: f64) -> Self {
        MemoryStats { percent }
    }
}

impl fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self.percent {
            p if p <= 50.0 => "\x1b[32m",
            p if p <= 75.0 => "\x1b[33m",
            _ => "\x1b[31m",
        };
        
        write!(
            f,
            "{}{:.1}%\x1b[0m",
            color,
            self.percent
        )
    }
}

pub(crate) fn get_memory_info() -> Option<MemoryStats> {
    let mem = mem_info().ok()?;
    #[cfg(unix)]
    let used = mem.total - mem.avail;
    
    #[cfg(windows)]
    let used = mem.total - mem.free;
    
    let percent = if mem.total > 0 {
        (used as f64 / mem.total as f64) * 100.0
    } else {
        0.0
    };

    Some(MemoryStats::new(percent))
}

pub(crate) fn get_swap_info() -> Option<MemoryStats> {
    #[cfg(unix)]
    {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("free -b | grep Swap")
            .output()
            .ok()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.split_whitespace().collect();
        
        if parts.len() >= 4 {
            let total = parts[1].parse().unwrap_or(0);
            let used = parts[2].parse().unwrap_or(0);
            let percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            return Some(MemoryStats::new(percent));
        } else {
            return None;
        }
    }

    #[cfg(windows)]
    {
        use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        use std::mem;

        let mut mem_status: MEMORYSTATUSEX = unsafe { mem::zeroed() };
        mem_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;

        if unsafe { GlobalMemoryStatusEx(&mut mem_status) } == 0 {
            return None;
        }

        let total_swap = mem_status.ullTotalPageFile - mem_status.ullTotalPhys;
        let used_swap = mem_status.ullTotalPageFile - mem_status.ullAvailPageFile - mem_status.ullAvailPhys;
        
        let percent = if total_swap > 0 {
            (used_swap as f64 / total_swap as f64)
        } else {
            0.0
        };

        Some(MemoryStats::new(percent))
    }
}
