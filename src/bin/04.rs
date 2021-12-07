use std::collections::HashSet;
use std::io::{self, Read};
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
enum MainError {
    #[error("No draw numbers")]
    NoDrawNumbers,
}

fn print_board(board: &Board) {
    for line in &board.data.iter().zip(board.marked.iter()).chunks(5) {
	for (num,&mark) in line {
	    let markchr = if mark { '*' } else { ' ' };
	    print!("{}{} ", num, markchr);
	}
	println!();
    }
}

fn find_num(board: &Board, num: i32) -> Option<usize> {
    board.data.iter().position(|&x| x == num)
}

fn find_bingo(board: &Board) -> Option<Vec<i32>> {
    let horizontal = board.marked.iter()
	.enumerate()
	.chunks(5)
	.into_iter()
        .find_map(|l| {
	    let v = l.filter(|(_,&b)| b).collect_vec();
	    if v.len() == 5 {
		Some(v)
	    } else {
		None
	    }
	});

    let vertical = (0..5).into_iter()
        .map(|i| 
	     (i,
	      board.marked.iter()
	      .enumerate()
	      .skip(i)
	      .step_by(5)
	      .filter(|(_,&b)| b)
	      .collect_vec()))
        .filter(|(_,v)| v.len() == 5)
        .map(|(_,v)| v)
	.next();

    // let diag_tlbr = board.marked.iter()
    // 	.enumerate()
    // 	.step_by(6)
    //     .take(5)
    //     .filter(|(_,&b)| b)
    //     .collect_vec();

    // let diag_tlbr = if diag_tlbr.len() == 5 {
    // 	Some(diag_tlbr)
    // } else {
    // 	None
    // };

    // let diag_trbl_v = board.marked.iter()
    // 	.enumerate()
    //     .skip(4)
    // 	.step_by(4)
    //     .take(5)
    //     .filter(|(_,&b)| b)
    //     .collect_vec();

    // let diag_trbl = if diag_trbl_v.len() == 5 {
    // 	Some(diag_trbl_v)
    // } else {
    // 	None
    // };

    let idxs = horizontal
	.or(vertical)
	// .or(diag_tlbr)
	// .or(diag_trbl)
	.map(|v| v
	     .into_iter()
	     .map(|(i,_)| i)
	     .collect_vec());

    idxs.map(|idxs| {
	let vals = board.data.iter()
	    .enumerate()
	    .filter(|(i,_)| idxs.contains(i))
	    .map(|(_,v)| *v)
	    .collect_vec();

	vals
    })
}

struct Board {
    data: Vec<i32>,
    marked: Vec<bool>
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let mut input = input_str
	.lines()
	.map(|s| s.trim())
        .filter(|s| !s.is_empty())
	.map(|s| s.to_string());

    let draw_numbers = input.next()
	.ok_or(MainError::NoDrawNumbers)?
	.split(',')
	.map(|s| s.parse())
	.collect::<Result<Vec<i32>,_>>()?;

    let mut boards : Vec<Board> = input
	.chunks(5)
        .into_iter()
	.map(|chunk| chunk
	     .map(|line| line
		  .split_whitespace()
		  .map(|n| n.parse::<i32>())
		  .collect::<Result<_,_>>())
	     .collect::<Result<_,_>>())
        .collect::<Result<Vec<Vec<Vec<i32>>>,_>>()?
	.into_iter()
        .map(|board| board.into_iter().flatten().collect())
        .map(|data| Board { data, marked: vec![false; 25] })
        .collect();

    println!("{:?}", draw_numbers);
    
    let mut last_winning_boards = HashSet::new();

    for &num in &draw_numbers {
	let idxs : Vec<(usize, usize)> = boards.iter()
	    .enumerate()
	    .filter_map(|(i,b)| find_num(b, num).map(|u| (i,u)))
	    .collect();

	// mark found position in all boards that contained number
	for (idx, pos) in idxs {
	    boards[idx].marked[pos] = true;
	}

	let winning_boards = boards.iter()
	    .enumerate()
	    .filter_map(|(i,board)| find_bingo(board).map(|v| (i,v)))
	    .collect_vec();

	/* part 1:
	if let Some((idx,v)) = winning_boards.get(0) {
	    println!("");
	    println!("Board {} is the winner with numbers: {:?}!", idx, v);
	    print_board(&boards[*idx]);

	    let unmarked_num_sum :i32 = boards[*idx].marked.iter()
		.enumerate()
		.filter(|(i,&b)| !b)
		.map(|(i,_)| boards[*idx].data[i])
		.sum();

	    println!("unmarked_num_sum * num = {}", unmarked_num_sum * num);

	}
	*/

	// part 2:
	let curr_winning_boards = winning_boards.iter().map(|(i,_)| *i).collect::<HashSet<_>>();
	let new_winning_boards = curr_winning_boards.difference(&last_winning_boards).cloned().collect::<HashSet<_>>();
	last_winning_boards = curr_winning_boards;

	if winning_boards.len() == boards.len() {
	    if let Some(idx) = new_winning_boards.iter().next() {
		if let Some(v) = winning_boards.iter().filter(|(i,_v)| i == idx).map(|(_,v)| v).next() {
		    println!();
		    println!("last picked number: {}", num);
		    println!("Board {} is the last winner with numbers: {:?}!", idx, v);
		    print_board(&boards[*idx]);

		    let unmarked_num_sum :i32 = boards[*idx].marked.iter()
			.enumerate()
			.filter(|(_,&b)| !b)
			.map(|(i,_)| boards[*idx].data[i])
			.sum();

		    println!("unmarked_num_sum * num = {}", unmarked_num_sum * num);

		    break;

		}
	    }
	}

    }

    Ok(())
}
