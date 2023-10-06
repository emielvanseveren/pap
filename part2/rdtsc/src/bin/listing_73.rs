use rdtsc::{read_cpu_timer, read_os_timer};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let usage = "Usage: listing_73 <ms_to_wait>";
    assert_eq!(args.len(), 2, "{}", usage);

    let ms_to_wait: u64 = args[1].parse::<u64>()?;
    let os_freq = rdtsc::get_os_timer_freq();
    println!("  OS Freq: {} (reported)", os_freq);

    let cpu_start = read_cpu_timer();
    let os_start = read_os_timer();
    let mut os_end = 0;
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

    println!(
        "  OS Timer: {} -> {} = {} elapsed",
        os_start, os_end, os_elapsed
    );
    println!("  OS Seconds: {}", os_elapsed / os_freq);
    println!(
        "  CPU Timer: {} -> {} = {} elapsed",
        cpu_start, cpu_end, cpu_elapsed
    );
    println!("  CPU Freq: {} (calculated)", cpu_freq);

    Ok(())
}
