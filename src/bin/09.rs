use itertools::Itertools;
use std::io::{self, Read};
use thiserror::*;

#[derive(Error,Debug)]
enum MainError {
    #[error("no input")]
    NoInput,
}

fn print_map(dims: (usize,usize), map: &[i32]) {
    let map_str = map.chunks(dims.0)
	.map(|chunk| chunk.iter()
	     .map(|n| format!("{}", n))
	     .join(""))
	.join("\n");

    println!("{}", map_str);
}

fn is_local_min(idx: usize, dims: (usize,usize), map: &[i32]) -> bool {
    let x = idx % dims.0;
    let y = idx / dims.0;

    let val = map[idx];

    let pnt_above = (y > 0).then(|| (x, y-1));
    let pnt_below = (y < dims.1 - 1).then(|| (x, y+1));
    let pnt_left = (x > 0).then(|| (x-1, y));
    let pnt_right = (x < dims.0 - 1).then(|| (x+1, y));
 
    let above = pnt_above.and_then(|(x,y)| map.get(y * dims.0 + x));
    let below = pnt_below.and_then(|(x,y)| map.get(y * dims.0 + x));
    let left = pnt_left.and_then(|(x,y)| map.get(y * dims.0 + x));
    let right = pnt_right.and_then(|(x,y)| map.get(y * dims.0 + x));


    let lower_than_above = above.map(|v| val < *v);
    let lower_than_below = below.map(|v| val < *v);
    let lower_than_left = left.map(|v| val < *v);
    let lower_than_right = right.map(|v| val < *v);


    [lower_than_above, lower_than_below, lower_than_left, lower_than_right]
        .into_iter()
        .map(|b| b.unwrap_or(true))
        .all(|b| b)
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let input = input_str
	.lines()
	.map(|s| s.trim())
        .filter(|s| !s.is_empty())
	.collect_vec();

    if input.len() == 0 { Err(MainError::NoInput)?; }

    let map_width = input[0].len();
    let map_height = input.len();

    let dims = (map_width, map_height);

    dbg!(&dims);

    let map : Vec<i32> = input.iter()
	.flat_map(|s| s.split(""))
        .filter(|s| !s.is_empty())
	.map(|s| s.parse::<i32>())
	.collect::<Result<_,_>>()?;

    print_map((map_width,map_height), &map);

    let lowest_points = map.iter().enumerate().filter(|(i,n)| is_local_min(*i, dims, &map)).collect_vec();

    println!("{:?}", lowest_points);

    println!("sum: {}", lowest_points.iter().map(|(_,n)| **n + 1).sum::<i32>());

    Ok(())
}
