use mach::mach_time::mach_absolute_time;
use mach::mach_time::mach_timebase_info;
extern crate mach;

#[derive(Default, Debug)]
enum TestMode {
    #[default]
    Uninitialized,
    Testing,
    Completed,
    Error,
}
#[derive(Default, Debug)]
pub struct RepetitionTestResults {
    test_count: u64,
    total_time: u64,
    max_time: u64,
    min_time: u64,
}

#[derive(Default, Debug)]
pub struct RepetitionTester {
    target_processed_byte_count: u64,
    cpu_timer_freq: u64,
    try_for_time: u64,
    tests_started_at: u64,

    mode: TestMode,
    print_new_minimums: bool,
    open_block_count: u32,
    close_block_count: u32,
    time_accumulated_on_this_test: u64,
    bytes_accumulated_on_this_test: u64,

    results: RepetitionTestResults,
}

pub fn seconds_from_cpu_time(cpu_time: f64, cpu_timer_freq: u64) -> f64 {
    cpu_time / cpu_timer_freq as f64
}

pub fn print_time(label: &str, cpu_time: f64, cpu_timer_freq: u64, byte_count: u64) {
    print!("{label}: {:.0}", cpu_time);

    if cpu_timer_freq != 0 {
        let seconds = seconds_from_cpu_time(cpu_time, cpu_timer_freq);
        print!(" ({:.3}ms)", 1000.0 * seconds);

        if byte_count != 0 {
            let gigabyte = 1024.0 * 1024.0 * 1024.0;
            let best_bandwidth = byte_count as f64 / (gigabyte * seconds);
            print!(" {:.3}gb/s", best_bandwidth);
        }
    }

    println!(); // add newline at the end like typical logging
}

fn read_cpu_timer() -> u64 {
    unsafe { mach_absolute_time() }
}

impl RepetitionTester {
    fn print_results(self, cpu_timer_freq: u64, byte_count: u64) {
        print_time(
            "Min",
            self.results.min_time as f64,
            cpu_timer_freq,
            byte_count,
        );
        print_time(
            "Max",
            self.results.max_time as f64,
            cpu_timer_freq,
            byte_count,
        );
        print_time(
            "Avg",
            (self.results.total_time / self.results.test_count) as f64,
            cpu_timer_freq,
            byte_count,
        );
    }

    fn begin_time(&mut self) {
        self.open_block_count += 1;
        self.time_accumulated_on_this_test -= read_cpu_timer();
    }

    fn end_time(&mut self) {
        self.close_block_count += 1;
        self.time_accumulated_on_this_test += read_cpu_timer();
    }

    fn count_bytes(&mut self, byte_count: u64) {
        self.bytes_accumulated_on_this_test += byte_count;
    }

    fn new_test_wave(target_processed_byte_count: u64, cpu_timer_freq: u64, seconds_to_try: u64) -> RepetitionTester {
        RepetitionTester {
            mode: TestMode::Testing,
            target_processed_byte_count,
            cpu_timer_freq,
            try_for_time: seconds_to_try * cpu_timer_freq,
            tests_started_at: read_cpu_timer(),
            ..Default::default()
        }
    }
}
