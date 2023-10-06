use std::time::SystemTime;

pub fn get_os_timer_freq() -> u64 {
    1_000_000
}

pub fn read_os_timer() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards");

    let in_micros = since_the_epoch.as_secs() + since_the_epoch.subsec_micros() as u64;
    get_os_timer_freq() * in_micros
}

pub fn read_cpu_timer() -> u64 {
    // this will of course only work on x86_64 CPUs (which is what I'm using)
    unsafe { std::arch::x86_64::_rdtsc() }
}

// // TODO Listing 74: Create a new function estimate_cpu_timer_freq
pub fn estimate_cpu_timer_freq() -> u64 {
    let ms_to_wait = 100;
    let os_freq = get_os_timer_freq();
    let cpu_start = read_cpu_timer();
    let os_start = read_os_timer();
    let mut os_end;
    let mut os_elapsed = 0;
    let os_wait_time = os_freq * ms_to_wait / 1000;
    while os_elapsed < os_wait_time {
        os_end = read_os_timer();
        os_elapsed = os_end - os_start;
    }

    let cpu_end = read_cpu_timer();
    let cpu_elapsed = cpu_end - cpu_start;

    let mut cpu_freq = 0;
    if os_elapsed > 0 {
        cpu_freq = os_freq * cpu_elapsed / os_elapsed;
    }

    cpu_freq
}
