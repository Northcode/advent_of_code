use std::io::{self, Read};
use std::cmp::Ordering;
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
enum MainError {
    #[error("no input")]
    NoInput,

    #[error("no oxygen")]
    NoOxygenRating,

    #[error("no co2")]
    NoCO2Rating,

    #[error("could not parse {0} as binary number")]
    InvalidBinary(String),
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let input = input_str
	.split("\r\n")
	.map(|s| s.trim())
	.filter(|s| !s.is_empty())
	.map(|s| s.to_string())
	.collect::<Vec<String>>();

    let code_length = input.first().ok_or(MainError::NoInput)?.chars().count();

    let input_length = input.len();

    let mut counts = vec![0i32; code_length];

    for item in input.iter() {
	for (i,c) in item.chars().enumerate() {
	    let p : i32 = match c {
		'1' => 1,
		_ => 0
	    };
	    counts[i] += p;
	}
    }

    dbg!(&counts);

    let gamma_str = counts.iter()
	.map(|i| *i > (input_length / 2) as i32)
	.map(|b| if b { 1 } else { 0 })
	.map(|i| i.to_string())
	.collect::<String>();

    let gamma = i32::from_str_radix(&gamma_str, 2)?;

    println!("{} ({})", gamma_str, gamma);

    let epsilon_str = counts.iter()
	.map(|i| *i < (input_length / 2) as i32)
	.map(|b| if b { 1 } else { 0 })
	.map(|i| i.to_string())
	.collect::<String>();

    let epsilon = i32::from_str_radix(&epsilon_str, 2)?;

    println!("{} ({})", epsilon_str, epsilon);

    let power = gamma * epsilon;

    println!("{}", power);

    let mut oxygen_candidates = input.clone();
    let mut co2_candidates = input.clone();

    let mut oxygen_rating_str = String::new();

    fn get_rating(from: &Vec<String>, bit: char) -> Option<String> {
	let mut candidates = from.clone();

	let code_length = candidates[0].len();
	
	for i in 0..code_length {

	    let count = candidates.len() as i32;

	    let current_bit_1_count = candidates.iter()
		.filter_map(|line| line.chars().nth(i))
		.filter(|c| *c == bit)
		.count()
		as i32;

	    let current_bit_0_count = count - current_bit_1_count;

	    let current_bit_mask = match current_bit_1_count.cmp(&current_bit_0_count) {
		Ordering::Less => '0',
		Ordering::Equal => bit,
		Ordering::Greater => '1'
	    };

	    candidates = candidates.into_iter()
		.filter(|line| line.chars().nth(i).unwrap() == current_bit_mask)
		.collect::<Vec<_>>();
	    
	    if candidates.len() == 1 {
		return Some(candidates[0].clone());
	    }
	}
	return None;
    }

    let oxygen_rating_str = get_rating(&input, '1').ok_or(MainError::NoOxygenRating)?;

    dbg!(&oxygen_rating_str);

    let oxygen_rating = i32::from_str_radix(&oxygen_rating_str, 2)?;

    dbg!(oxygen_rating);

    let co2_rating_str = get_rating(&input, '0').ok_or(MainError::NoOxygenRating)?;

    dbg!(&co2_rating_str);

    let co2_rating = i32::from_str_radix(&co2_rating_str, 2)?;

    dbg!(co2_rating);

    let life_rating = oxygen_rating * co2_rating;

    dbg!(life_rating);

    Ok(())
}
