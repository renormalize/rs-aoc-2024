use std::{collections::HashMap, fs};

/// read each row into its own vector of integers
fn read_input_into_vectors(file_name: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let (mut one, mut two): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    let _: () = input
        .lines()
        .map(|line| {
            let mut r = line.split_whitespace();
            one.push(
                r.next()
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .expect("parse error for first number"),
            );
            two.push(
                r.next()
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .expect("parse error for second number"),
            );
        })
        .collect();
    one.sort();
    two.sort();
    (one, two)
}

pub fn solve_part_1(file_name: &str) {
    let (first, second) = read_input_into_vectors(file_name);
    let pair = first.iter().zip(second.iter());
    let mut distance = 0;
    pair.for_each(|(a, b)| distance += i32::abs(*a - *b));
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 1 :: Solving part 1 for {input_type} distance is:\t{distance}");
}

pub fn solve_part_2(file_name: &str) {
    let (first, second) = read_input_into_vectors(file_name);
    // create a map of the frequencies
    let mut freq: HashMap<i32, i32> = HashMap::new();
    second.iter().for_each(|x| {
        *freq.entry(*x).or_default() += 1;
    });
    let mut sum = 0;
    first.iter().for_each(|x| {
        sum += *x * *freq.get(x).unwrap_or(&0);
    });
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 1 :: Solving part 2 for {input_type} distance is:\t{sum}");
}
