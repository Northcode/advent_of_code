use std::io::{self, Read};
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
enum CommandParseError {
    #[error("no direction")]
    NoDirection,
    #[error("no amount")]
    NoAmount
}

#[derive(Error, Debug)]
enum CommandError {
    #[error("invalid command {0}")]
    InvalidCommand(String),
}

fn parse_command(s: impl AsRef<str>) -> Result<(String,i32), anyhow::Error> {

    let mut parts = s.as_ref().split(' ');

    let direction = parts.next().ok_or(CommandParseError::NoDirection)?.to_string();
    let amount = parts.next().ok_or(CommandParseError::NoAmount)?.parse()?;
    
    return Ok((direction, amount));
}

#[derive(Debug)]
struct State {
    horizontal: i32,
    depth: i32,
    aim: i32
}

fn apply_command(state: State, command: &(String, i32)) -> Result<State, CommandError> {
    let amount = command.1;
    match command.0.as_str() {
	"forward" => Ok(State { horizontal: state.horizontal + amount, depth: state.depth + state.aim * amount, ..state }),
	"down" => Ok(State { aim: state.aim + amount, ..state }),
	"up" => Ok(State { aim: state.aim - amount, ..state }),
	_ =>  Err(CommandError::InvalidCommand(command.0.clone())),
    }
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

    let commands = input.iter()
	.map(parse_command)
	.collect::<Result<Vec<_>,_>>()?;

    let initial_state = State { horizontal: 0, depth: 0, aim: 0 };

    let end_state = commands.iter()
	.try_fold(initial_state, apply_command)?;

    println!("{}", end_state.horizontal * end_state.depth);

    Ok(())
}

