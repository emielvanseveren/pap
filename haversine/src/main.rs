use anyhow::Result;
use clap::Parser;
use rand::prelude::*;
use std::fs;
use std::io::Write;

#[derive(Parser, Clone, Debug)]
enum PointGeneration {
    Uniform,
    Cluster,
}

impl From<String> for PointGeneration {
    fn from(s: String) -> Self {
        match s.as_str() {
            "uniform" => Self::Uniform,
            "cluster" => Self::Cluster,
            _ => panic!("Invalid point generation method"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Method of the point generation
    #[arg(long)]
    method: PointGeneration,

    /// Seed for the random number generator
    #[arg(long)]
    seed: u64,

    /// Number of points to generate
    #[arg(long)]
    pair_count: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let points = match args.method {
        PointGeneration::Uniform => generate_uniform_points(args.pair_count, args.seed),
        PointGeneration::Cluster => generate_cluster_points(args.pair_count, args.seed),
    };

    // calculate expected sum (take 2 points, calculate haversine distance, sum it, take average)

    let mut sum = 0.0;
    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in points[i + 1..].iter() {
            sum += haversine(x1, y1, x2, y2, 6372.8);
        }
    }
    let expected_sum = sum / (points.len() * (points.len() - 1) / 2) as f64;

    let mut output = String::new();
    output.push_str(&format!("Method: {:?}\n", args.method));
    output.push_str(&format!("Seed: {:?}\n", args.seed));
    output.push_str(&format!("Pair count: {:?}\n", args.pair_count));
    output.push_str(&format!("Expected sum: {:?}\n", expected_sum));

    std::io::stdout().write_all(output.as_bytes())?;

    parse_points_to_json(&points, "points.json")?;

    Ok(())
}

// lat1, lon1, lat2, lon2 in degrees
// x = lon, y = lat
// lon between -180 and 180
// lat between -90 and 90
fn generate_uniform_points(n: u64, seed: u64) -> Vec<(f64, f64)> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let x = rng.gen::<f64>() * 360.0 - 180.0;
        let y = rng.gen::<f64>() * 180.0 - 90.0;
        points.push((x, y));
    }
    points
}

// { pairs: [ {"x0": 0.0, "y0": 0.0, "x1": 0.0, "y1": 0.0 } ] }
fn parse_points_to_json(points: &[(f64, f64)], filename: &str) -> Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(b"{\"pairs\":[")?;

    for pairs in points.chunks(2) {
        if pairs.len() == 2 {
            let (x0, y0) = pairs[0];
            let (x1, y1) = pairs[1];
            file.write_all(
                format!(
                    "{{\"x0\":{},\"y0\":{},\"x1\":{},\"y1\":{}}}",
                    x0, y0, x1, y1
                )
                .as_bytes(),
            )?;
            if pairs != &points[points.len() - 2..] {
                file.write_all(b",")?;
            }
        }
    }

    file.write_all(b"]}")?;
    Ok(())
}

// similar to uniform but with clusters
fn generate_cluster_points(n: u64, seed: u64) -> Vec<(f64, f64)> {
    unimplemented!()
}

// This is not meant to be a good implementation of the haversine distance calculation
fn haversine(lon1: f64, lat1: f64, lon2: f64, lat2: f64, radius: f64) -> f64 {
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = f64::powi(f64::sin(dlat / 2.0), 2)
        + lat1.cos() * lat2.cos() * f64::powi(f64::sin(dlon / 2.0), 2);

    let c = 2.0 * f64::asin(f64::sqrt(a));

    radius * c
}
