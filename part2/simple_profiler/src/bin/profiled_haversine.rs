use haversine::haversine;
use json_parser::{parse_json_str, parser::JsonValue};
use simple_profiler::{ScopeGuard, PROFILE};

fn read_json_input(file_path: &str) -> String {
    let _ = ScopeGuard::start("read_json_input");
    std::fs::read_to_string(file_path).expect("file to exist")
}

fn calculate_haversine_sum(json: &JsonValue) -> f64 {
    let _ = ScopeGuard::start("calculate_haversine_sum");
    if let JsonValue::Object { kv } = &json {
        if let Some((_, JsonValue::Array { values })) = kv.iter().find(|(k, _)| *k == "pairs") {
            let mut sum = 0.0;
            for pair in values {
                if let JsonValue::Object { kv } = pair {
                    let x0 = get_number_from_kv(kv, "x0");
                    let y0 = get_number_from_kv(kv, "y0");
                    let x1 = get_number_from_kv(kv, "x1");
                    let y1 = get_number_from_kv(kv, "y1");
                    sum += haversine(x0, y0, x1, y1, None);
                }
            }
            sum
        } else {
            panic!("'pairs' key does not contain an array");
        }
    } else {
        panic!("Expected object");
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    PROFILE.lock().unwrap().start();

    let args = std::env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 2, "Usage: <file_path>");
    let json_input = read_json_input(&args[1]);

    // parse the json
    let s = ScopeGuard::start("parse_json_str");
    let result = parse_json_str(&json_input).expect("valid json");
    std::mem::drop(s);

    // calculate the haversine sum
    let sum = calculate_haversine_sum(&result);
    println!("Total haversine distance: {:.2}", sum);

    let mut p = PROFILE.lock().unwrap();
    p.end();
    p.report();

    Ok(())
}

fn get_number_from_kv(kv: &[(&str, JsonValue)], key: &str) -> f64 {
    if let Some((_, JsonValue::Number(val))) = kv.iter().find(|(k, _)| *k == key) {
        *val
    } else {
        panic!("Expected number for {}", key);
    }
}
