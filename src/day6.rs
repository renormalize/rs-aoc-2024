use std::fs;

#[derive(Debug, Clone)]
enum PositionFreedom {
    Blocked,
    Allowed,
}

#[derive(Debug, Clone)]
struct Graph {
    freedom: Vec<Vec<PositionFreedom>>,
    start: (usize, usize),
    len: (usize, usize),
}

impl Graph {
    fn generate_graph(text: &str) -> Self {
        let mut freedom = Vec::<Vec<PositionFreedom>>::new();
        let mut start = (0, 0);
        text.lines().enumerate().for_each(|(line_number, line)| {
            freedom.push(
                line.chars()
                    .enumerate()
                    .map(|(char_index, c)| {
                        if c == '#' {
                            PositionFreedom::Blocked
                        } else if c == '^' {
                            start = (line_number, char_index);
                            PositionFreedom::Allowed
                        } else {
                            PositionFreedom::Allowed
                        }
                    })
                    .collect::<Vec<PositionFreedom>>(),
            );
        });
        let bounds = (freedom.len(), freedom[0].len());
        Graph {
            freedom,
            start,
            len: bounds,
        }
    }
    // count_visited assumes that the guard will not get stuck in a loop, and does no checks to ensure
    // that an infinite loop is not entered while computing the total number of visited positions
    fn count_visited(&self) -> usize {
        let mut visited = vec![vec![false; self.freedom[0].len()]; self.freedom.len()];
        // the start is already visited
        visited[self.start.0][self.start.1] = true;
        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut curr_direction_index = 0usize;
        let (mut i, mut j): (i32, i32) = (self.start.0 as i32, self.start.1 as i32);
        // assuming that the start position is valid
        loop {
            // check bounds after the newer position is calculated
            // try continuing in the same direction
            // if it is not possible, change the direction
            (i, j) = next_position((i, j), directions[curr_direction_index]);
            if !check_bounds((i, j), self.len.0, self.len.1) {
                break;
            }
            match self.freedom[i as usize][j as usize] {
                PositionFreedom::Blocked => {
                    // change the direction
                    (i, j) = previous_position((i, j), directions[curr_direction_index]);
                    curr_direction_index = (curr_direction_index + 1) % directions.len();
                }
                PositionFreedom::Allowed => {
                    // mark visited
                    visited[i as usize][j as usize] = true;
                }
            }
        }
        let visited_count = visited.iter().fold(0usize, |acc, element| {
            acc + element.iter().fold(0usize, |acc, &b| acc + (b as usize))
        });
        visited_count
    }
    fn count_loop(&self) -> usize {
        let mut total_count = 0usize;
        // brute force by blocking each position and checking for cycles
        for i in 0..self.len.0 {
            for j in 0..self.len.1 {
                let mut graph = self.clone();
                graph.freedom[i][j] = PositionFreedom::Blocked;

                // visited marks if a position is visited, and the direction in which it is visited
                let mut visited =
                    vec![vec![[false; 4]; graph.freedom[0].len()]; graph.freedom.len()];
                let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
                let mut curr_direction_index = 0usize;

                visited[graph.start.0][graph.start.1][0] = true;
                let (mut i, mut j): (i32, i32) = (graph.start.0 as i32, graph.start.1 as i32);
                // assuming that the start position is valid
                loop {
                    // check bounds after the newer position is calculated
                    // try continuing in the same direction
                    // if it is not possible, change the direction
                    (i, j) = next_position((i, j), directions[curr_direction_index]);
                    if !check_bounds((i, j), graph.len.0, graph.len.1) {
                        break;
                    }
                    match graph.freedom[i as usize][j as usize] {
                        PositionFreedom::Blocked => {
                            // change the direction
                            (i, j) = previous_position((i, j), directions[curr_direction_index]);
                            curr_direction_index = (curr_direction_index + 1) % directions.len();
                        }
                        PositionFreedom::Allowed => {
                            // current direction already visited, loop detected
                            if visited[i as usize][j as usize][curr_direction_index] {
                                total_count += 1;
                                break;
                            }
                            // mark visited
                            visited[i as usize][j as usize][curr_direction_index] = true;
                        }
                    }
                }
            }
        }
        total_count
    }
}

fn check_bounds(position: (i32, i32), x_len: usize, y_len: usize) -> bool {
    position.0 >= 0 && position.0 < x_len as i32 && position.1 >= 0 && position.1 < y_len as i32
}

fn next_position(position: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    (position.0 + direction.0, position.1 + direction.1)
}

fn previous_position(position: (i32, i32), diretion: (i32, i32)) -> (i32, i32) {
    (position.0 - diretion.0, position.1 - diretion.1)
}

pub fn solve_part_1(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");

    let graph = Graph::generate_graph(&contents);
    let visited_count = graph.count_visited();

    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 6 :: Solving part 1 for {input_type} count:\t\t{visited_count}");
}

pub fn solve_part_2(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");

    let graph = Graph::generate_graph(&contents);
    // to conclusively prove that the loop has begun, the guard must be
    // at a given location, moving in the same direction more than once
    // maintain a list of directions the guard has visited a block in
    // if the direction is already present in the list, then the guard
    // has started looping around
    let total_count = graph.count_loop();

    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 6 :: Solving part 2 for {input_type} count:\t\t{total_count}");
}
