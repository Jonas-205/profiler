// examples/profiler.rs

struct Dummy {
    _a: u32,
    _b: u32,
}

fn sleep_ms(ms: u32) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}

impl Dummy {
    fn new(a: u32, b: u32) -> Dummy {
        profiler::time_init!();

        sleep_ms(100);
        {
            profiler::time_init!("Other");
            sleep_ms(150);
        }
        sleep_ms(10);

        Dummy { _a: a, _b: b }
    }

    fn run(&self) {
        for _i in 0..100 {
            profiler::time_run!("Other");
            sleep_ms(10);
            {
                profiler::time_run!("Other 1");
                sleep_ms(1);

                {
                    profiler::time_run!("Other 1.1");
                    sleep_ms(1);
                }

                sleep_ms(1);
            }
            {
                profiler::time_run!("Other 2");
                sleep_ms(3);
            }
        }
    }
}

impl Drop for Dummy {
    fn drop(&mut self) {
        profiler::time_free!();
        sleep_ms(100);
    }
}

fn main() {
    profiler::begin_profiling("Example");
    {
        // dummy::drop has to be called before profiler::end_profiling
        let dummy = Dummy::new(1, 2);
        dummy.run();
    }
    profiler::end_profiling();
}
