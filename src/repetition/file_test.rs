use std::{fs::File, io::Read, os::unix::fs::MetadataExt};

use super::RepetitionTester;

pub fn file_read_test() {
    let mut file = File::open("output/data_10000.json").unwrap();
    let size = file.metadata().unwrap().size();

    let mut tester = RepetitionTester::new_test_wave(size, 10);
    while tester.is_still_testing() {
        tester.begin_time();
        let mut buffer = vec![0u8; size as usize];
        let _ = file.read(&mut buffer);
        tester.end_time();
        tester.count_bytes(size);
    }
    tester.print_results();
}
