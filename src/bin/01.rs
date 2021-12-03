use std::io::{self,Read};
use itertools::Itertools;

#[allow(dead_code)]
fn part1(input: &[i32]) {
    let is_bigger = input.iter()
        .tuple_windows()
        .filter(|(prev,current)| current > prev);

    let larger_count = is_bigger.count();

    println!("{}", larger_count);
}

fn part2(input: &[i32]) {

    let sums = input.iter()
        .tuple_windows()
        .map(|(prev, current, next)| next + current + prev)
        .collect::<Vec<i32>>();
    
    let is_bigger = sums.iter()
        .tuple_windows()
        .filter(|(prev, current)| current > prev);

    let larger_count = is_bigger.count();

    println!("{}", larger_count);
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let input = input_str
	.split("\r\n")
	.map(|s| s.trim())
	.filter(|s| !s.is_empty())
	.map(|s| s.parse())
	.collect::<Result<Vec<i32>,_>>()?;
    
    part2(&input);
    
    Ok(())
}
