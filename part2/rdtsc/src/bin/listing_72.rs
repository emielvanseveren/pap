use rdtsc::{get_os_timer_freq, read_cpu_timer, read_os_timer};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let os_freq = get_os_timer_freq();
    println!("  OS Freq: {}", os_freq);

    let cpu_start = read_cpu_timer();
    let os_start = read_os_timer();
    let mut os_end = 0;
    let mut os_elapsed = 0;

    while os_elapsed < os_freq {
        os_end = read_os_timer();
        os_elapsed = os_end - os_start;
    }

    let cpu_end = read_cpu_timer();
    let cpu_elapsed = cpu_end - cpu_start;

    println!(
        "  OS Timer: {} -> {} = {} elapsed",
        os_start, os_end, os_elapsed
    );
    println!("  OS Seconds: {}", os_elapsed / os_freq);
    println!(
        "  CPU Timer: {} -> {} = {} elapsed",
        cpu_start, cpu_end, cpu_elapsed
    );

    Ok(())
}
