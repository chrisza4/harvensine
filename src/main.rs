use std::path::Path;

use repetition::{file_test::file_read_test, print_time, read_cpu_timer};

mod calc;
mod generator;
mod json;
mod repetition;

fn main() {
    file_read_test();
}
