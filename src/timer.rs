use std::time::SystemTime;

use crate::{instructor, TimingMode};

pub struct Timer {
    name: Option<String>,
    start: SystemTime,
    mode: TimingMode,
}

impl Timer {
    pub fn new(name: &str, mode: TimingMode) -> Timer {
        Timer {
            name: Some(name.to_string()),
            start: SystemTime::now(),
            mode,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let stop = SystemTime::now();
        instructor::add_result(&self.name.take().unwrap(), self.start, stop, &self.mode);
    }
}
