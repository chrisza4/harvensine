use std::{fs::File, os::unix::fs::MetadataExt};

use super::RepetitionTester;


pub fn file_read_test() {
    let mut file = File::open("output/data_10000.json").unwrap();
    let size = file.metadata().unwrap().size();
    
    // let tester = RepetitionTester::new_test_wave(size, cpu_timer_freq, seconds_to_try)
    let mut buffer = vec![0u8; size as usize];

    // let bytes_read = file.read(&mut buffer)?; // Like fread()

    // println!("Read {} bytes", bytes_read);

    // let actual_data = &buffer[..bytes_read];
}
