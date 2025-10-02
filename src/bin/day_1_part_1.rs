use anyhow::Result;
use aoc2023::{launch_solver, Solver};

struct Day1Part1Solver();

impl Solver for Day1Part1Solver {
    fn solve(input_data: String) -> u32 {
        let data_lines = input_data.lines();
        let mut sum = 0;
        for data_line in data_lines {
            let mut sub_sum = Vec::<u32>::new();
            for data_bytes in data_line.as_bytes() {
                match { *data_bytes as char }.to_digit(10) {
                    Some(num) => {
                        sub_sum.push(num);
                    }
                    None => {}
                }
            }
            if sub_sum.len() == 1 {
                let final_number = format!("{}{}", sub_sum[0], sub_sum[0])
                    .parse::<u32>()
                    .expect("Can't create final number");
                sum = sum + final_number;
            } else {
                let last_element = sub_sum.pop().expect("Got empty Vec");
                let final_number = format!("{}{}", sub_sum[0], last_element)
                    .parse::<u32>()
                    .expect("Can't create final number");
                sum = sum + final_number;
            }
        }
        sum
    }
}
fn main() -> Result<()> {
    launch_solver::<Day1Part1Solver>()
}
