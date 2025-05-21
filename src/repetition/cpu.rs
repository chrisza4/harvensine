#[cfg(unix)]
mod timer {
    use libc::{gettimeofday, timeval};

    pub fn get_os_timer_freq() -> u64 {
        1_000_000 // microseconds
    }

    pub fn read_os_timer() -> u64 {
        unsafe {
            let mut tv = timeval { tv_sec: 0, tv_usec: 0 };
            gettimeofday(&mut tv, std::ptr::null_mut());
            (tv.tv_sec as u64 * 1_000_000) + tv.tv_usec as u64
        }
    }
}

pub fn estimate_cpu_timer_freq() -> u64 {
    let ms_to_wait = 100;
    let os_freq = timer::get_os_timer_freq();

    let cpu_start = read_cpu_timer();
    let os_start = timer::read_os_timer();

    let os_wait_time = os_freq * ms_to_wait / 1000;
    let mut os_elapsed;
    loop {
        let os_end = timer::read_os_timer();
        os_elapsed = os_end - os_start;
        if os_elapsed >= os_wait_time {
            break;
        }
    }

    let cpu_end = read_cpu_timer();
    let cpu_elapsed = cpu_end - cpu_start;

    if os_elapsed != 0 {
        os_freq * cpu_elapsed / os_elapsed
    } else {
        0
    }
}
