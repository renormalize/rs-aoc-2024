use regex::{Captures, Regex};
use std::fs;

fn nums_from_capture(capture: Captures<'_>) -> (i32, i32) {
    (
        capture
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .expect("could not parse the first match to an integer"),
        capture
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .expect("could not parse the second match to an integer"),
    )
}

pub fn solve_part_1(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let syntax = Regex::new(r"mul\((\d+)\,(\d+)\)").expect("regex could not be compiled");
    let mut sum = 0;
    for capture in syntax.captures_iter(&contents) {
        let (num1, num2) = nums_from_capture(capture);
        sum += num1 * num2;
    }
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 3 :: Solving part 1 for {input_type} sum:\t{sum}");
}

pub fn solve_part_2(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let syntax =
        Regex::new(r"mul\((\d+)\,(\d+)\)|do\(\)|don\'t\(\)").expect("regex could not be compiled");
    let mut state = true;
    let mut sum = 0;
    for capture in syntax.captures_iter(&contents) {
        let y = capture.get(0).unwrap().as_str();
        if y == "don't()" {
            state = false;
            continue;
        } else if y == "do()" {
            state = true;
            continue;
        }
        if state {
            let (num1, num2) = nums_from_capture(capture);
            sum += num1 * num2;
        }
    }
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 3 :: Solving part 2 for {input_type} sum:\t{sum}");
}
