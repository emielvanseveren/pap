use haversine::haversine;
use json_parser::{parse_json_str, parser::JsonValue};
use rdtsc::read_cpu_timer;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 2, "Usage: <file_path>");

    // profile
    let prof_end = 0;

    let prof_begin = read_cpu_timer();

    let file_path = &args[1];
    let prof_read = read_cpu_timer();
    let json_input = std::fs::read_to_string(file_path).expect("file to exist");
    let prof_misc_setup = read_cpu_timer();

    // parse the json
    let prof_parse = read_cpu_timer();
    let result = parse_json_str(&json_input).expect("valid json");
    let prof_sum = read_cpu_timer();
    // calculate the haversine distance sum
    let pairs = if let JsonValue::Object { kv } = &result {
        kv
    } else {
        panic!("Expected object");
    };
    let (_, value) = pairs
        .iter()
        .find(|(k, _)| *k == "pairs")
        .unwrap_or_else(|| {
            panic!("Didn't find 'pairs'");
        });

    let mut sum = 0.0;
    if let JsonValue::Array { values } = value {
        for pair in values {
            if let JsonValue::Object { kv } = pair {
                let x0 = get_number_from_kv(kv, "x0");
                let y0 = get_number_from_kv(kv, "y0");
                let x1 = get_number_from_kv(kv, "x1");
                let y1 = get_number_from_kv(kv, "y1");

                sum += haversine(x0, y0, x1, y1, None);
            }
        }
    } else {
        panic!("Expected array");
    }
    let prof_end = read_cpu_timer();
    let prof_total = prof_end - prof_begin;

    print_time_elapsed("Startup", prof_total, prof_begin, prof_read);
    print_time_elapsed("Read", prof_total, prof_read, prof_misc_setup);
    print_time_elapsed("MiscSetup", prof_total, prof_misc_setup, prof_parse);
    print_time_elapsed("Parse", prof_total, prof_parse, prof_sum);
    print_time_elapsed("Sum", prof_total, prof_sum, prof_end);

    println!("Total haversine distance: {:.2}", sum);
    Ok(())
}

fn get_number_from_kv(kv: &[(&str, JsonValue)], key: &str) -> f64 {
    if let Some((_, JsonValue::Number(val))) = kv.iter().find(|(k, _)| *k == key) {
        *val
    } else {
        panic!("Expected number for {}", key);
    }
}

fn print_time_elapsed(label: &str, total: u64, begin: u64, end: u64) {
    let elapsed = end - begin;
    let percent = 100.0 * elapsed as f64 / total as f64;
    println!("{}: {} {:.2}%", label, elapsed, percent);
}
