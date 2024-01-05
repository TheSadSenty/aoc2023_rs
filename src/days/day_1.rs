use std::fs::read;
pub fn solve() {
    let data = String::from_utf8(
        read("/home/sentyyy/projects/aoc2023/input/day_1.txt").expect("Can't read day_1.txt"),
    )
    .expect("Can't make String from Vec");
    let data_lines = data.lines();
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
    println!("Day 1 part 1: {}", sum);
}
