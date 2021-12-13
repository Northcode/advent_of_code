use itertools::Itertools;
use std::collections::HashSet;
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
	     .map(|n| format!("{}", match n { 9 => '#',
					      -1 => '.',
					      11 => 'A',
					      12 => 'B',
					      13 => 'C',
					      14 => 'D',
					      15 => 'E',
					      16 => 'F',
					      _ => ' ' }))
	     .join(""))
	.join("\n");

    println!("{}", map_str);
}

fn is_local_min(idx: usize, dims: (usize,usize), map: &[i32]) -> bool {

    let val = map[idx];

    get_sides(idx, dims, map)
        .into_iter()
        .map(|o| o.map(|(_,v)| val < v))
        .map(|b| b.unwrap_or(true))
        .all(|b| b)
}

fn get_sides(idx: usize, dims: (usize,usize), map: &[i32]) -> [Option<(usize,i32)>; 4] {
    let (x,y) = idx_to_coords(idx, dims);

    let pnt_above = (y > 0).then(|| (x, y-1));
    let pnt_below = (y < dims.1 - 1).then(|| (x, y+1));
    let pnt_left = (x > 0).then(|| (x-1, y));
    let pnt_right = (x < dims.0 - 1).then(|| (x+1, y));

    fn f((x,y): (usize,usize), dims: (usize,usize), map: &[i32]) -> Option<(usize,i32)> {
	let idx = coords_to_idx((x,y), dims);
	let val = map.get(y * dims.0 + x)?;

	Some((idx,*val))
    }
 
    let above = pnt_above.and_then(|(x,y)| f((x,y),dims,map));
    let below = pnt_below.and_then(|(x,y)| f((x,y),dims,map));
    let left  = pnt_left.and_then(|(x,y)| f((x,y),dims,map));
    let right = pnt_right.and_then(|(x,y)| f((x,y),dims,map));

    [above, below, left, right]
}

fn idx_to_coords(idx: usize, dims: (usize,usize)) -> (usize,usize) {
    let x = idx % dims.0;
    let y = idx / dims.0;

    (x,y)
}

fn coords_to_idx(coords: (usize,usize), dims: (usize,usize)) -> usize {
    coords.1 * dims.0 + coords.0
}

// Horribly bad, but oh well
fn flood_fill(idx: usize, dims: (usize, usize), map: &[i32]) -> HashSet<usize> {
    let mut set = HashSet::new();
    let mut stk = Vec::new();

    set.insert(idx);
    stk.push(idx);

    let mut idx = idx;

    loop {
	let sides = get_sides(idx, dims, map);

	let next_side = sides.into_iter()
	    .filter(|o| o.map(|(_,v)| v < 9).unwrap_or(true))
	    .filter(|o| o.map(|(i,_)| !set.contains(&i)).unwrap_or(false))
	    .next();

	if let Some(o) = next_side {
	    if let Some((i,_)) = o {
		set.insert(i);
		stk.push(idx);
		idx = i;
	    } else {
		if let Some(i) = stk.pop() {
		    idx = i;
		} else {
		    break;
		}
	    }
	} else {
	    if let Some(i) = stk.pop() {
		idx = i;
	    } else {
		break;
	    }
	}
    }

    set
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

    if input.is_empty() { return Err(MainError::NoInput.into()); }

    let map_width = input[0].len();
    let map_height = input.len();

    let dims = (map_width, map_height);

    dbg!(&dims);

    let mut map : Vec<i32> = input.iter()
	.flat_map(|s| s.split(""))
        .filter(|s| !s.is_empty())
	.map(|s| s.parse::<i32>())
	.collect::<Result<_,_>>()?;


    let lowest_points = map.iter().cloned().enumerate().filter(|(i,_)| is_local_min(*i, dims, &map)).collect_vec();

    for (i,_) in lowest_points.iter() {
	map[*i] = -1;
    }

    print_map((map_width,map_height), &map);

    println!("{:?}", lowest_points);

    println!("sum: {}", lowest_points.iter().map(|(_,n)| *n + 1).sum::<i32>());

    let mut regions = lowest_points.iter().map(|(i,_)| flood_fill(*i, dims, &map)).collect_vec();

    regions.sort_by_key(|set| set.len());
    regions.reverse();


    let largest = regions.iter().take(6);

    for (set,n) in largest.zip([11,12,13,14,15,16]) {
	for i in set {
	    map[*i] = n;
	}
    }


    print_map((map_width,map_height), &map);

    let sizes = regions.iter().map(|m| m.len()).collect_vec();

    dbg!(sizes.iter().take(3).product::<usize>());

    Ok(())
}
