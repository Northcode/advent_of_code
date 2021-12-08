use itertools::Itertools;
use std::io::{self, Read};
use thiserror::*;

#[derive(Error,Debug)]
enum MainError {
    #[error("invalid point")]
    InvalidLine,
    
    #[error("invalid point")]
    InvalidPoint,

    #[error("no input")]
    NoInput,
}

struct Line {
    from: (i32,i32),
    to: (i32,i32)
}

fn parse_point(s: &str) -> Result<(i32,i32), anyhow::Error> {
    let coords = s.split_once(',').ok_or(MainError::InvalidPoint)?;
    let x_coord = coords.0.parse::<i32>();
    let y_coord = coords.1.parse::<i32>();

    Ok((x_coord?, y_coord?))
}

fn points_between(from: (i32,i32), to: (i32,i32)) -> Option<Vec<(i32,i32)>> {

    let diff_x = (from.0 - to.0).abs();
    let diff_y = (from.1 - to.1).abs();

    let is_valid = (diff_x == 0 || diff_y == 0) || (diff_x == diff_y); // either x or y must match for it to be a line

    if !is_valid { return None; }
    
    /*
    10001
    00000
    00000
    00000
    10001
    00000
    */

    let points = if diff_x == diff_y {
	if from.0 < to.0 {
	    if from.1 < to.1 {
		(from.0..=to.0).zip(from.1..=to.1).collect_vec()
	    }
	    else {
		(from.0..=to.0).zip((to.1..=from.1).rev()).collect_vec()
	    }
	} else {
	    if from.1 < to.1 {
		(to.0..=from.0).rev().zip(from.1..=to.1).collect_vec()
	    }
	    else {
		(to.0..=from.0).rev().zip((to.1..=from.1).rev()).collect_vec()
	    }
	}
    } else {
	let start = (from.0.min(to.0),from.1.min(to.1));
	let end = (start.0 + diff_x + 1, start.1 + diff_y + 1);

	(start.0..end.0)
	    .flat_map(move |x| {
		(start.1..end.1)
		    .map(move |y| {
			(x,y)
		    })
	    })
	    .collect_vec()
    };
    
    Some(points)
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let input = input_str
	.lines()
	.map(|s| s.trim())
        .filter(|s| !s.is_empty())
	.map(|s| {
	    let (from,to) = s.split_once(" -> ").ok_or(MainError::InvalidLine)?;

	    let from = parse_point(from)?;
	    let to = parse_point(to)?;

	    Ok(Line { from, to })
	})
        .collect::<Result<Vec<Line>,anyhow::Error>>()?;

    let largest_x = 1 + input.iter().map(|line| line.from.0.max(line.to.0)).max().ok_or(MainError::NoInput)? as usize;
    let largest_y = 1 + input.iter().map(|line| line.from.1.max(line.to.1)).max().ok_or(MainError::NoInput)? as usize;

    dbg!(largest_x, largest_y);

    let mut board = vec![0i32; largest_x * largest_y];

    for line in input {
	// dbg!(line.from,line.to);
	if let Some(points) = points_between(line.from, line.to) {
	    // println!("{:?}", points);
	    for point in points {
		board[(point.1 as usize * largest_y) + point.0 as usize] += 1;
	    }
	}

	// print_board((largest_x, largest_y), &board);
	// println!();
    }

    print_board((largest_x, largest_y), &board);

    let overlap_count = board.iter().filter(|n| **n > 1).count();

    dbg!(overlap_count);

    Ok(())
}

fn print_board(dims: (usize,usize), board: &[i32]) {
    let s = board.chunks(dims.1)
        .map(|line| line.iter().map(|n| n.to_string()).join(" "))
        .join("\n");

    println!("{}", s);
}
