use std::fs::read;
pub fn solve() {
    let data = String::from_utf8(
        read("/home/sentyyy/projects/aoc2023/input/day_2.txt").expect("Can't read day_1.txt"),
    )
    .expect("Can't make String from Vec");
    let data_lines = data.lines();
    let mut sum = 0;
    // iter throughout each line
    for (index, data_line) in data_lines.enumerate() {
        //red, green, blue
        let games = data_line.split(": ").nth(1).unwrap();
        let mut cond_violation = false;
        // iter throughout each set of cubes
        for cube_set in games.split("; ") {
            let mut cubes_colors = [0, 0, 0];
            // iter throughout each cube in set of cubes
            for cubes in cube_set.split(", ") {
                let num = cubes.split(" ").next().unwrap().parse::<i32>().unwrap();
                let color = cubes.split(" ").nth(1).unwrap();
                match color {
                    "red" => {
                        cubes_colors[0] = cubes_colors[0] + num;
                    }
                    "green" => {
                        cubes_colors[1] = cubes_colors[1] + num;
                    }
                    "blue" => {
                        cubes_colors[2] = cubes_colors[2] + num;
                    }
                    _ => {
                        panic!()
                    }
                }
            }
            if cubes_colors[0] > 12 || cubes_colors[1] > 13 || cubes_colors[2] > 14 {
                cond_violation = true;
                break;
            }
        }
        if !cond_violation {
            sum = sum + index + 1;
        }
    }
    println!("Day 2: {}", sum);
}
