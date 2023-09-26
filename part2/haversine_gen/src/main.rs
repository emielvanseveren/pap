use anyhow::Result;
use haversine::haversine;
use rand_casey::{random_in_range, seed, RandomSeries};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

struct PointPair {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let usage = "Usage: haversine_gen [uniform/cluster] [random seed] [num coords to gen]";
    assert_eq!(args.len(), 4, "{}", usage);

    let gen_type = args[1].parse::<String>()?.to_lowercase();
    let seed = args[2].parse::<u64>()?;
    let n = args[3].parse::<u64>()?;

    let point_pairs = match gen_type.as_str() {
        "uniform" => gen_polar_coords_uniform(seed, n),
        "cluster" => gen_polar_coords_cluster(seed, n),
        _ => {
            println!("{}", usage);
            std::process::exit(1);
        }
    };

    let haversine_vals: Vec<f64> = point_pairs
        .iter()
        .map(|pp| haversine(pp.x0, pp.y0, pp.x1, pp.y1, None))
        .collect();
    let num_pairs_f64 = n as f64;
    let haversine_mean = haversine_vals
        .iter()
        .fold(0.0, |acc, x| acc + (x / num_pairs_f64));

    write_point_pairs_json(&point_pairs).expect("Failed to write data to json file.");
    write_haversine_binary_file(&haversine_vals, haversine_mean)
        .expect("Failed to write data to binary file.");

    println!(
        "\
        Method: {}\n\
        Random Seed: {}\n\
        Pair Count: {}\n\
        Haversine Mean: {}",
        gen_type, seed, n, haversine_mean
    );
    Ok(())
}

fn gen_polar_coords_uniform(random_seed: u64, num_pairs: u64) -> Vec<PointPair> {
    let mut point_pairs = Vec::<PointPair>::with_capacity(num_pairs as usize);
    let mut random_series = seed(random_seed);

    for _ in 0..num_pairs {
        point_pairs.push(rand_point_pair_in_range(
            &mut random_series,
            -180.0,
            -90.0,
            180.0,
            90.0,
        ));
    }

    point_pairs
}

fn write_haversine_binary_file(
    haversine_vals: &Vec<f64>,
    haversine_mean: f64,
) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(format!(
        "data_{}_haveranswer.f64",
        haversine_vals.len()
    ))?);

    for haversine_val in haversine_vals {
        writer.write_all(&haversine_val.to_le_bytes())?;
    }

    writer.write_all(&haversine_mean.to_le_bytes())?;
    writer.flush()?;

    Ok(())
}

fn write_point_pairs_json(point_pairs: &Vec<PointPair>) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(format!(
        "data_{}_flex.json",
        point_pairs.len()
    ))?);

    // header
    write!(writer, "{{\"pairs\":[")?;

    if !point_pairs.is_empty() {
        // Handle first element separately as to not have a trailing comma (json doesn't accept that)
        let first_pair = &point_pairs[0];
        write!(
            writer,
            "\n\t{{\"x0\":{},\"y0\":{},\"x1\":{},\"y1\":{}}}",
            first_pair.x0, first_pair.y0, first_pair.x1, first_pair.y1
        )?;

        for pp in &point_pairs[1..] {
            write!(
                writer,
                ",\n\t{{\"x0\":{},\"y0\":{},\"x1\":{},\"y1\":{}}}",
                pp.x0, pp.y0, pp.x1, pp.y1
            )?;
        }
    }

    // footer
    writeln!(writer, "\n]}}")?;
    writer.flush()?;

    Ok(())
}

fn rand_point_pair_in_range(
    random_series: &mut RandomSeries,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> PointPair {
    PointPair {
        x0: random_in_range(random_series, min_x, max_x),
        y0: random_in_range(random_series, min_y, max_y),
        x1: random_in_range(random_series, min_x, max_x),
        y1: random_in_range(random_series, min_y, max_y),
    }
}

fn gen_polar_coords_cluster(random_seed: u64, num_pairs: u64) -> Vec<PointPair> {
    let mut point_pairs = Vec::<PointPair>::with_capacity(num_pairs as usize);

    let mut random_series_cluster_range = seed(random_seed);
    let mut random_series_point_pairs = seed(random_seed);

    let mut push_cluster_coords = |count: u64| {
        let rand_point_pair =
            rand_point_pair_in_range(&mut random_series_cluster_range, -180.0, -90.0, 180.0, 90.0);

        let min_x = rand_point_pair.x0.min(rand_point_pair.x1);
        let max_x = rand_point_pair.x0.max(rand_point_pair.x1);
        let min_y = rand_point_pair.y0.min(rand_point_pair.y1);
        let max_y = rand_point_pair.y0.max(rand_point_pair.y1);

        for _ in 0..count {
            point_pairs.push(rand_point_pair_in_range(
                &mut random_series_point_pairs,
                min_x,
                min_y,
                max_x,
                max_y,
            ));
        }
    };

    let num_pair_partitions = 5;
    let size_of_clusters = num_pairs / num_pair_partitions;
    let size_of_last_cluster = num_pairs % num_pair_partitions;

    for _ in 0..num_pair_partitions {
        push_cluster_coords(size_of_clusters);
    }
    push_cluster_coords(size_of_last_cluster);

    point_pairs
}
