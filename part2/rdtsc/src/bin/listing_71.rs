use rdtsc::get_os_timer_freq;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let os_freq = get_os_timer_freq();
    println!("  OS Freq: {}", os_freq);

    let os_start = rdtsc::read_os_timer();
    let mut os_end: u64 = 0;
    let mut os_elapsed = 0;

    while os_elapsed < os_freq {
        os_end = rdtsc::read_os_timer();
        os_elapsed = os_end - os_start;
    }

    println!(
        "  OS Timer: {} -> {} = {} elapsed",
        os_start, os_end, os_elapsed
    );
    println!("  OS Seconds: {}", os_elapsed / os_freq);
    Ok(())
}
