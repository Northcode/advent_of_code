use itertools::Itertools;
use std::io::{self, Read};
use std::collections::HashSet;
use thiserror::*;

#[derive(Error,Debug)]
enum MainError {
    #[error("no input")]
    NoInput,
    #[error("invalid input")]
    InvalidInput,
}

fn print_sheet(dims: &(usize,usize), coords: &[(usize,usize)]) {
    let mut map = vec!['.'; dims.0 * dims.1];

    for (x,y) in coords {
	map[y * dims.0 + x] = '#';
    }
    
    let map_str = map.chunks(dims.0)
	.map(|chunk| chunk.iter()
	     .map(|n| format!("{}", n))
	     .join(""))
	.join("\n");

    println!("{}", map_str);
}

#[derive(Debug)]
struct Instruction {
    axis: char,
    idx: usize
}

fn parse_instruction(s: &str) -> Result<Instruction, anyhow::Error> {
    if !s.starts_with("fold along ") {
	anyhow::bail!("wrong start of instruction");
    }

    let axis = s["fold along ".len()..].chars().next().ok_or(MainError::InvalidInput)?;

    let idx : usize = s["fold along x=".len()..].trim().parse()?;
    
    let instruction = Instruction { axis, idx };

    Ok(instruction)
}

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();

    let mut input_str = String::new();
    stdin.lock().read_to_string(&mut input_str)?;

    let input = input_str
	.lines()
	.map(|s| s.trim())
	.collect_vec();

    let sheet = input.iter()
	.take_while(|s| !s.is_empty())
	.map(|s| -> Result<(usize,usize), anyhow::Error> {
	    let (x,y) = s.split_once(',').ok_or(MainError::InvalidInput)?;
	    let x : usize = x.parse()?;
	    let y : usize = y.parse()?;
	    Ok((x,y))
	})
	.collect::<Result<Vec<_>,_>>()?;

    let instructions = input.iter()
	.skip_while(|s| !s.is_empty())
	.skip(1)
        .map(|s| parse_instruction(s))
        .collect::<Result<Vec<_>,_>>()?;

    if sheet.is_empty() { return Err(MainError::NoInput.into()); }
    if instructions.is_empty() { return Err(MainError::NoInput.into()); }

    let dims = (1 + *sheet.iter().map(|(x,_)| x).max().expect("no sheet max x") as usize,
		1 + *sheet.iter().map(|(_,y)| y).max().expect("no sheet max y") as usize);

    dbg!(&sheet);
    dbg!(&dims);
    println!("{:?}", &instructions);

    print_sheet(&dims, &sheet);

    println!();

    let mut tmp_dims = dims;
    let mut tmp_sheet = sheet;

    for instruction in instructions.iter() {
	if instruction.axis == 'x' {
	    for (x,_) in &mut tmp_sheet {
		*x = instruction.idx - (*x as i32 - instruction.idx as i32).abs() as usize;
	    }
	    tmp_dims.0 = tmp_dims.0 - instruction.idx - 1;
	}
	else if instruction.axis == 'y' {
	    for (_,y) in &mut tmp_sheet {
		*y = instruction.idx - (*y as i32 - instruction.idx as i32).abs() as usize;
	    }
	    tmp_dims.1 = tmp_dims.1 - instruction.idx - 1;
	}
    }

    print_sheet(&tmp_dims, &tmp_sheet);

    let visible_dots = tmp_sheet.into_iter().collect::<HashSet<(usize,usize)>>();

    dbg!(visible_dots.len());

    Ok(())
}
