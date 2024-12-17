use std::{fs, ops::Range};

#[derive(Clone, Copy, Debug)]
struct Repeated {
    state: i32,
    value: i32,
}

impl Repeated {
    fn new(value: i32, state: i32) -> Self {
        Self { value, state }
    }
}

impl Iterator for Repeated {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state > 0 {
            self.state -= 1;
            Some(self.value)
        } else {
            None
        }
    }
}

// IndexChar is a zipped Iterator that holds an iterator to the x,y position as the first iterator
// and the corresponding character that should be present there as the second iterator
type IndexCharZip<'a> = std::iter::Zip<std::iter::Zip<Indices, Indices>, std::str::Chars<'a>>;

#[derive(Clone, Debug)]
enum Indices {
    // Increasing range
    Ranges(Range<i32>),
    // Decreasing range
    RevRanges(Range<i32>),
    // Same values
    Same(Repeated),
}

impl Iterator for Indices {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Ranges(r) => r.next(),
            Self::RevRanges(r) => r.next_back(),
            Self::Same(s) => s.next(),
        }
    }
}

struct CharacterPositions {
    item: String,
    origin: (usize, usize),
}

impl CharacterPositions {
    fn new(input: &str, i: usize, j: usize) -> Self {
        CharacterPositions {
            item: input.to_string(),
            origin: (i, j),
        }
    }
    // all 8 directions are inherently returned by the method
    fn positions_origin(&self) -> Vec<IndexCharZip<'_>> {
        // fn positions_origin(&self) -> Vec<std::iter::Zip<std::iter::Zip<R, R>, std::str::Chars<'_>>> {
        // generate the indices of all characters for one direction
        let x_indices = [
            Indices::Ranges(self.origin.0 as i32..(self.origin.0 as i32 + self.item.len() as i32)),
            Indices::RevRanges(
                (self.origin.0 as i32 - self.item.len() as i32 + 1)..(self.origin.0 as i32 + 1),
            ),
            Indices::Same(Repeated::new(self.origin.0 as i32, self.item.len() as i32)),
        ];

        let y_indices = [
            Indices::Ranges(self.origin.1 as i32..(self.origin.1 as i32 + self.item.len() as i32)),
            Indices::RevRanges(
                (self.origin.1 as i32 - self.item.len() as i32 + 1)..(self.origin.1 as i32 + 1),
            ),
            Indices::Same(Repeated::new(self.item.len() as i32, self.origin.1 as i32)),
        ];

        let mut a = vec![];
        for x in &x_indices {
            for y in &y_indices {
                a.push(x.clone().zip(y.clone()).zip(self.item.chars()));
            }
        }

        // Remove the last element where neither x nor y change
        a.pop();
        a
    }
    // this method will only work with odd strings
    fn positions_diagonal(&self) -> Vec<std::iter::Zip<IndexCharZip<'_>, IndexCharZip<'_>>> {
        // four possible X variants
        let x_inc_start = self.origin.0 as i32 - self.item.len() as i32 / 2;
        let x_dec_start = self.origin.0 as i32 + self.item.len() as i32 / 2;
        let x_indices = [
            // positive i
            Indices::Ranges(x_inc_start..(x_inc_start + self.item.len() as i32)),
            // negative i
            Indices::RevRanges((x_dec_start - self.item.len() as i32 + 1)..(x_dec_start + 1)),
        ];

        let y_inc_start = self.origin.1 as i32 - self.item.len() as i32 / 2;
        let y_dec_start = self.origin.1 as i32 + self.item.len() as i32 / 2;
        let y_indices = [
            // positive j
            Indices::Ranges(y_inc_start..(y_inc_start + self.item.len() as i32)),
            // negative j
            Indices::RevRanges((y_dec_start - self.item.len() as i32 + 1)..(y_dec_start + 1)),
        ];
        let mut a = vec![];
        for x in &x_indices {
            for y in &y_indices {
                a.push(x.clone().zip(y.clone()).zip(self.item.chars()));
            }
        }

        // 0: +i +j
        // 1: +i -j
        // 2: -i +j
        // 3: -i -j

        // +i +j, +i -j
        // +i -j, -i -j
        // -i -j, -i +j
        // -i +j, +i +j

        vec![
            a[0].clone().zip(a[1].clone()),
            a[1].clone().zip(a[3].clone()),
            a[3].clone().zip(a[2].clone()),
            a[2].clone().zip(a[0].clone()),
        ]
    }
}

fn in_bounds(x: i32, y: i32, x_limit: usize, y_limit: usize) -> bool {
    x >= 0 && x < x_limit as i32 && y >= 0 && y < y_limit as i32
}

pub fn solve_part_1(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let mut mat: Vec<Vec<char>> = vec![];
    contents
        .lines()
        .for_each(|line| mat.push(line.chars().collect()));
    let mut count = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let x = CharacterPositions::new("XMAS", i, j);
            let v = x.positions_origin();
            for mut vv in v {
                if vv.all(|x| {
                    in_bounds(x.0 .0, x.0 .1, mat.len(), mat[i].len())
                        && mat[x.0 .0 as usize][x.0 .1 as usize] == x.1
                }) {
                    count += 1;
                }
            }
        }
    }
    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 4 :: Solving part 1 for {input_type} count:\t\t{count}");
}

pub fn solve_part_2(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let mut mat: Vec<Vec<char>> = vec![];
    contents
        .lines()
        .for_each(|line| mat.push(line.chars().collect()));
    let mut count = 0;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            let x = CharacterPositions::new("MAS", i, j);
            let v = x.positions_diagonal();
            for mut vv in v {
                if vv.all(|(x, y)| {
                    in_bounds(x.0 .0, x.0 .1, mat.len(), mat[i].len())
                        && mat[x.0 .0 as usize][x.0 .1 as usize] == x.1
                        && in_bounds(y.0 .0, y.0 .1, mat.len(), mat[i].len())
                        && mat[y.0 .0 as usize][y.0 .1 as usize] == y.1
                }) {
                    count += 1;
                }
            }
        }
    }

    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 4 :: Solving part 2 for {input_type} count:\t\t{count}");
}
