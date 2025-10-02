use anyhow::{Context, Ok, Result};
use std::{env, fs::File, io::Read};
pub trait Solver {
    fn solve(input_data: String) -> u32;
}

pub fn main_solver<N: Solver>() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args
        .get(1)
        .with_context(|| format!("Miss file path ot task input data"))?;
    let mut file = File::open(file_path).with_context(|| format!("Can't open file {file_path}"))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .with_context(|| format!("Can't read file {file_path}"))?;

    let result = N::solve(contents);
    println!("Result: {}", result);

    Ok(())
}
