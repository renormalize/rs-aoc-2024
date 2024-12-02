use std::fs;

fn lines_to_levels_iter(contents: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    contents.lines().map(|line| {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|word| {
                word.trim()
                    .parse::<i32>()
                    .expect("parse error while converting to i32")
            })
            .collect();
        values
    })
}

fn check_safety_all_levels(levels: Vec<i32>) -> bool {
    levels
        .windows(2)
        .all(|w| w[1] > w[0] && w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
        || levels
            .windows(2)
            .all(|w| w[0] > w[1] && w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
}

fn check_safety_one_bad_level(levels: Vec<i32>) -> bool {
    if !check_safety_all_levels(levels.clone()) {
        for i in 0..levels.len() {
            let mut levels_c = levels.clone();
            levels_c.remove(i);
            if check_safety_all_levels(levels_c) {
                return true;
            }
        }
        false
    } else {
        true
    }
}

pub fn solve_part_1(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let levels_iter = lines_to_levels_iter(&contents);
    let mut safe = 0;
    levels_iter.for_each(|levels| {
        if check_safety_all_levels(levels) {
            safe += 1;
        }
    });
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 2 :: Solving part 1 for {input_type} safe levels:\t{safe}");
}

pub fn solve_part_2(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let levels_iter = lines_to_levels_iter(&contents);
    let mut safe: i32 = 0;
    levels_iter.for_each(|levels| {
        if check_safety_one_bad_level(levels) {
            safe += 1;
        }
    });
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 2 :: Solving part 2 for {input_type} safe levels:\t{safe}");
}
