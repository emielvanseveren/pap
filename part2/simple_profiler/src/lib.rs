use lazy_static::lazy_static;
use rdtsc::read_cpu_timer;
use std::collections::hash_map::HashMap;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    pub static ref PROFILE: Mutex<SimpleProfiler> = Mutex::new(SimpleProfiler {
        start_tsc: 0,
        end_tsc: 0,
        scopes: HashMap::new()
    });
}

/// To get the actual start and end of of the program we cannot rely on the drop mechanic since
/// that will only be called when the program goes out of scope. So for that timing specifically we
/// need a separate start and end timing.
pub struct SimpleProfiler {
    /// Start time of timestamp counter
    pub start_tsc: u64,
    /// End time of timestamp counter
    pub end_tsc: u64,
    pub scopes: HashMap<&'static str, u64>,
}

impl SimpleProfiler {
    pub fn start(&mut self) {
        self.start_tsc = read_cpu_timer();
    }
    pub fn end(&mut self) {
        self.end_tsc = read_cpu_timer();
    }

    pub fn update_scope_time(&mut self, label: &'static str, new_time: u64) {
        if let Some(start_time) = self.scopes.get_mut(label) {
            *start_time = new_time;
        }
    }

    pub fn report(&mut self) {
        let total = self.end_tsc - self.start_tsc;
        println!("Report");
        println!("=====================");
        println!("Total time: {}", total);

        for (label, elapsed) in &self.scopes {
            let percent = 100.0 * (*elapsed) as f64 / total as f64;
            println!("{}: {} {:.2}%", label, elapsed, percent);
        }
    }
}

pub struct ScopeGuard(&'static str);

impl ScopeGuard {
    pub fn start(label: &'static str) -> Self {
        let mut p = PROFILE.lock().unwrap();
        p.scopes.insert(label, read_cpu_timer());
        Self(label)
    }
    pub fn end(label: &'static str) {
        let mut p = PROFILE.lock().unwrap();
        let end = read_cpu_timer();
        let start: &mut u64 = p.scopes.get_mut(label).expect("anchor with label to exist");
        *start = end - *start;
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        let end_time = read_cpu_timer();
        let mut profiler = PROFILE.lock().unwrap();
        if let Some(start_time) = profiler.scopes.get(self.0) {
            if *start_time > profiler.start_tsc {
                let elapsed_time = end_time - *start_time;
                profiler.update_scope_time(self.0, elapsed_time);
            }
        }
    }
}
