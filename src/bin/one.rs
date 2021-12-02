use std::io::{self,Read};

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
    
    let prevs = input.iter();

    let is_bigger = input.iter()
	.skip(1)
	.zip(prevs)
        .map(|(current,prev)| current > prev);

    let larger_count = is_bigger.filter(|i| *i).count();

    println!("{}", larger_count);
    
    Ok(())
}
