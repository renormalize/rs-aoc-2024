mod day1;
mod day2;
mod day3;

fn file_path(day: i32) -> String {
    format!("input/day{}/input.txt", day)
}

fn sample_path(day: i32) -> String {
    format!("input/day{}/sample.txt", day)
}

pub fn solve_day_1() {
    // solve the sample to check if the code is working
    day1::solve_part_1(&sample_path(1));
    // solve the puzzle
    day1::solve_part_1(&file_path(1));
    // solve the sample to check if the code is working
    day1::solve_part_2(&sample_path(1));
    // solve the puzzle
    day1::solve_part_2(&file_path(1));
}

pub fn solve_day_2() {
    // solve the sample to check if the code is working
    day2::solve_part_1(&sample_path(2));
    // solve the puzzle
    day2::solve_part_1(&file_path(2));
    // solve the sample to check if the code is working
    day2::solve_part_2(&sample_path(2));
    // solve the puzzle
    day2::solve_part_2(&file_path(2));
}

pub fn solve_day_3() {
    // solve the sample to check if the code is working
    day3::solve_part_1(&sample_path(3));
    // solve the puzzle
    day3::solve_part_1(&file_path(3));
    // solve the sample to check if the code is working
    day3::solve_part_2(&sample_path(3));
    // solve the sample to check if the code is working
    day3::solve_part_2(&file_path(3));
}
