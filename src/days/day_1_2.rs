use std::{collections::BTreeMap, fs::read};
pub fn solve() {
    let list_of_nums = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let data = String::from_utf8(
        read("/home/sentyyy/projects/aoc2023/input/day_1.txt").expect("Can't read day_1.txt"),
    )
    .expect("Can't make String from Vec");
    let data_lines = data.lines();
    let mut sum = 0;
    for data_line in data_lines {
        let mut num_position = BTreeMap::<usize, &str>::new();
        for key in list_of_nums.keys() {
            match data_line.find(key) {
                Some(num_index) => {
                    num_position.insert(num_index, &key);
                }
                None => {}
            }
            match data_line.rfind(key) {
                Some(num_index) => {
                    num_position.insert(num_index, &key);
                }
                None => {}
            }
        }
        let mut final_num = 0;
        if let Some(first_num) = num_position.first_entry() {
            final_num = final_num
                + list_of_nums
                    .get(first_num.get())
                    .expect("Can't find value")
                    .clone()
                    * 10;
        }
        if let Some(last_num) = num_position.last_entry() {
            final_num = final_num
                + list_of_nums
                    .get(last_num.get())
                    .expect("Can't find value")
                    .clone();
        }
        sum = sum + final_num;
    }
    println!("Day 1 part 2: {}", sum);
}
