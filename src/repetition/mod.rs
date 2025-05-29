pub mod file_test;
use std::{cmp, mem, u64};

use mach::{
    kern_return::KERN_SUCCESS,
    mach_time::{mach_absolute_time, mach_timebase_info},
};
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
    total_time: i64,
    max_time: u64,
    min_time: u64,
}

#[derive(Default, Debug)]
pub struct CpuTimer {
    timebase_info: mach_timebase_info,
}

impl CpuTimer {
    pub fn new() -> CpuTimer {
        unsafe {
            let mut timebase_info = mem::zeroed();
            let result = mach_timebase_info(&mut timebase_info);
            if result != KERN_SUCCESS {
                panic!("Cannot get timebase info from CPU")
            }
            CpuTimer { timebase_info }
        }
    }
    pub fn nano_seconds_from_cpu_time(&self, elapsed_tick: u64) -> f64 {
        let numer = self.timebase_info.numer as f64;
        let denom = self.timebase_info.denom as f64;
        (elapsed_tick as f64) * (numer / denom)
    }
}

pub fn read_cpu_timer() -> u64 {
    unsafe { mach_absolute_time() }
}

#[derive(Default, Debug)]
pub struct RepetitionTester {
    try_for_time: u64,
    tests_started_at: u64,
    target_processed_byte_count: u64,
    cpu_timer: CpuTimer,

    mode: TestMode,
    print_new_minimums: bool,
    open_block_count: u32,
    close_block_count: u32,
    time_accumulated_on_this_test: i64,
    bytes_accumulated_on_this_test: u64,

    results: RepetitionTestResults,
}

pub fn print_time(label: &str, cpu_time: u64, cpu_timer: &CpuTimer, byte_count: u64) {
    let mut to_print = String::new();
    to_print.push_str(&format!("{label}: {:.0}", cpu_time));

    let milliseconds = cpu_timer.nano_seconds_from_cpu_time(cpu_time) / 1_000_000.0;
    to_print.push_str(&format!(" ({:.3}ms)", milliseconds));
    let seconds = milliseconds * 1000.0;

    if byte_count != 0 {
        let gigabyte = 1024.0 * 1024.0 * 1024.0;
        let best_bandwidth = byte_count as f64 / (gigabyte * seconds);
        to_print.push_str(&format!(" {:.3} gb/s", best_bandwidth));
    }
    to_print.push_str(" | ");
    print!("{}", to_print);
}

impl RepetitionTester {
    fn print_results(&self) {
        let byte_count = self.bytes_accumulated_on_this_test;
        print_time("Min", self.results.min_time, &self.cpu_timer, byte_count);
        print_time("Max", self.results.max_time, &self.cpu_timer, byte_count);
        print_time(
            "Avg",
            self.results.total_time as u64 / self.results.test_count,
            &self.cpu_timer,
            byte_count,
        );
        print!("\r");
    }

    fn begin_time(&mut self) {
        self.open_block_count += 1;
        self.time_accumulated_on_this_test -= read_cpu_timer() as i64;
    }

    fn end_time(&mut self) {
        self.close_block_count += 1;
        self.time_accumulated_on_this_test += read_cpu_timer() as i64;
    }

    fn count_bytes(&mut self, byte_count: u64) {
        self.bytes_accumulated_on_this_test += byte_count;
    }

    fn new_test_wave(target_processed_byte_count: u64, seconds_to_try: u64) -> RepetitionTester {
        RepetitionTester {
            mode: TestMode::Testing,
            target_processed_byte_count,
            cpu_timer: CpuTimer::new(),
            try_for_time: seconds_to_try,
            tests_started_at: read_cpu_timer(),
            results: RepetitionTestResults {
                test_count: 0,
                total_time: 0,
                max_time: 0,
                min_time: u64::MAX,
            },

            ..Default::default()
        }
    }

    fn is_still_testing(&self) -> bool {
        self.cpu_timer
            .nano_seconds_from_cpu_time(read_cpu_timer() - self.tests_started_at)
            / 1_000_000_000.0
            < self.try_for_time as f64
    }

    fn start_test_loop(&mut self) {
        let elapsed_time = self.time_accumulated_on_this_test as u64;
        self.results.total_time += self.time_accumulated_on_this_test;
        self.results.max_time = cmp::max(self.results.max_time, elapsed_time);
        if elapsed_time > 0 {
            self.results.min_time = cmp::min(self.results.min_time, elapsed_time);
        }
        self.time_accumulated_on_this_test = 0;
        self.open_block_count = 0;
        self.close_block_count = 0;
        self.bytes_accumulated_on_this_test = 0;
        self.results.test_count += 1;
    }
}
