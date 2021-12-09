use itertools::Itertools;
use std::io::{self, Read};
use thiserror::*;

#[derive(Error,Debug)]
enum MainError {
    #[error("no input")]
    NoInput,
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let fish = input_str
	.lines()
	.map(|s| s.trim())
        .find(|s| !s.is_empty())
        .ok_or(MainError::NoInput)?
        .split(',')
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>,_>>()?;

    let mut generations = vec![0, 1, 0, 0, 0, 0, 0, 0, 0];

    let days : usize = std::env::args()
	.nth(1)
	.unwrap_or_else(|| "256".to_string())
	.parse()?;

    let counts = (0..days).
	map(|_| {
	    let parents = generations[0];
	    generations.rotate_left(1);
	    generations[6] += parents;
	    generations[8] = parents;
	    generations.iter().sum::<i64>()
	})
        .skip(days - 7)
        .collect_vec();

    let total_fish : i64 = fish.iter().map(|f| counts[7 - *f as usize]).sum();

    println!("total fish: {}", total_fish);

    Ok(())
}
