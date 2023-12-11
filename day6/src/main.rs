use std::io::BufRead;
use aoc_utils::prelude::*;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let mut reader = get_puzzle_input()?.lines();

    let time_line = reader.next().expect("Error reading time line")?;
    let distance_line = reader.next().expect("Error reading distance line")?;

    let time_line = if cfg!(feature = "part2") {
        time_line.chars().filter(|c| !c.is_ascii_whitespace()).collect()
    }
    else {
        time_line
    };

    let distance_line = if cfg!(feature = "part2") {
        distance_line.chars().filter(|c| !c.is_ascii_whitespace()).collect()
    }
    else {
        distance_line
    };

    let times: Vec<u64> = time_line
        .strip_prefix("Time:")
        .expect("Invalid time line")
        .split_ascii_whitespace()
        .map(|s| u64::from_str_radix(s, 10))
        .collect::<Result<Vec<u64>,_>>()?;

    let distances: Vec<u64> = distance_line
        .strip_prefix("Distance:")
        .expect("Invalid distance line")
        .split_ascii_whitespace()
        .map(|s| u64::from_str_radix(s, 10))
        .collect::<Result<Vec<u64>,_>>()?;

    assert_eq!(times.len(), distances.len(), "Should be the same number of times and distances");

    let result: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            // calculate one of the roots
            let tf: f64 = *t as f64;
            let df: f64 = *d as f64;
            (t, ((tf - (tf.powi(2) - 4.0 * df).sqrt()) / 2.0 as f64 + 1.0) as u64)
        })
        .map(|(t, n)| {
            *t - 2u64 * n + 1u64
        })
        .product();

    println!("{:?}", result);
    Ok(())
}
