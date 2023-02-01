use std::{
    fs::File,
    io::{BufWriter, Write},
    process::exit,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::TimingMode;

use lazy_static::lazy_static;

lazy_static! {
    static ref S_INSTRUMENTOR_INIT: Mutex<Option<Instrumentor>> = Mutex::new(None);
    static ref S_INSTRUMENTOR_RUN: Mutex<Option<Instrumentor>> = Mutex::new(None);
    static ref S_INSTRUMENTOR_FREE: Mutex<Option<Instrumentor>> = Mutex::new(None);
}

pub fn end_session() {
    S_INSTRUMENTOR_INIT.lock().unwrap().take();
    S_INSTRUMENTOR_RUN.lock().unwrap().take();
    S_INSTRUMENTOR_FREE.lock().unwrap().take();
}

pub fn begin_session(session_name: &str) {
    let dir = format!("session_data/{session_name}");

    match std::fs::create_dir_all(&dir) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Failed to create directory: {dir}");
            exit(1);
        }
    };

    let mut i_init = Instrumentor::new(&format!("{dir}/init.json"));
    let mut i_run = Instrumentor::new(&format!("{dir}/run.json"));
    let mut i_free = Instrumentor::new(&format!("{dir}/free.json"));

    i_init.write_header();
    i_run.write_header();
    i_free.write_header();

    *S_INSTRUMENTOR_INIT.lock().unwrap() = Some(i_init);
    *S_INSTRUMENTOR_RUN.lock().unwrap() = Some(i_run);
    *S_INSTRUMENTOR_FREE.lock().unwrap() = Some(i_free);
}

pub fn add_result(name: &str, start: SystemTime, stop: SystemTime, mode: &TimingMode) {
    match mode {
        TimingMode::Init => {
            if let Some(i) = S_INSTRUMENTOR_INIT.lock().unwrap().as_mut() {
                i.write_profile(name, start, stop);
            }
        }
        TimingMode::Run => {
            if let Some(i) = S_INSTRUMENTOR_RUN.lock().unwrap().as_mut() {
                i.write_profile(name, start, stop);
            }
        }
        TimingMode::Free => {
            if let Some(i) = S_INSTRUMENTOR_FREE.lock().unwrap().as_mut() {
                i.write_profile(name, start, stop);
            }
        }
    }
}

struct Instrumentor {
    file_buffer: BufWriter<File>,
    count: usize,
    stopped: bool,
}

impl Instrumentor {
    fn new(filepath: &str) -> Instrumentor {
        let file = match File::create(filepath) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to create profiling file: {filepath}");
                exit(1);
            }
        };

        let file_buffer = BufWriter::with_capacity(512 * 1024, file);

        Instrumentor {
            file_buffer,
            count: 0,
            stopped: false,
        }
    }

    pub fn close(&mut self) {
        self.write_footer();
        self.file_buffer.flush().unwrap();
        self.stopped = true;
    }

    pub fn write_profile(&mut self, name: &str, start: SystemTime, stop: SystemTime) {
        if self.count > 0 {
            self.file_buffer.write_all(",".as_bytes()).unwrap();
        }
        self.count += 1;

        let name = name.replace('\"', "\\\"");
        let start = start.duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 / 1000.0;
        let stop = stop.duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64 / 1000.0;

        self.file_buffer.write_all(format!("{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":{}}}", stop - start, name, start).as_bytes()).unwrap();
    }

    fn write_header(&mut self) {
        self.file_buffer
            .write_all("{\"otherData\": {}, \"traceEvents\":[".as_bytes())
            .unwrap();
    }

    fn write_footer(&mut self) {
        self.file_buffer.write_all("]}".as_bytes()).unwrap();
    }
}

impl Drop for Instrumentor {
    fn drop(&mut self) {
        if !self.stopped {
            self.close();
        }
    }
}
