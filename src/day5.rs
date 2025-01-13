use std::{cmp::Ordering, fs, str::Lines};

#[derive(Default, Debug)]
enum Edge {
    From(usize),
    To(usize),
    #[default]
    No,
}

struct Graph([Vec<Edge>; 100]);

impl Graph {
    // Creates the graph from the Line input, which are edges
    // It mutates the Lines till the newline between the edges and nodes is provided
    fn create_graph(lines: &mut Lines) -> Self {
        let mut adj: [Vec<Edge>; 100] = std::array::from_fn(|_| vec![]);
        for line in lines {
            if line.is_empty() {
                break;
            }
            let (node_1, node_2) = line.split_once('|').unwrap();
            let (node_1, node_2) = (
                node_1.parse::<usize>().unwrap(),
                node_2.parse::<usize>().unwrap(),
            );
            adj[node_1].push(Edge::To(node_2));
            adj[node_2].push(Edge::From(node_1));
        }
        Graph(adj)
    }
    // from_edges returns all nodes that point to node
    fn get_from_edges(&'_ self, node: usize) -> impl Iterator<Item = usize> + '_ {
        let edges = &self.0[node];
        let x = edges.iter().filter_map(|e| {
            if let Edge::From(ed) = e {
                Some(*ed)
            } else {
                None
            }
        });
        x
    }
    // to_edges returns all nodes that node points to
    fn get_to_edges(&'_ self, node: usize) -> impl Iterator<Item = usize> + '_ {
        let edges = &self.0[node];
        edges.iter().filter_map(|e| {
            if let Edge::To(ed) = *e {
                Some(ed)
            } else {
                None
            }
        })
    }
    // This DFS implementation assumes that there are no cycles in the graph
    // This method is not used to solve since the graph contains cycles
    // The solution also does not require transitivity checks
    #[allow(dead_code)]
    fn topologically_sorted(&self) -> Vec<usize> {
        let mut visited = [false; 100];
        let mut list = Vec::<usize>::new();

        fn internal_dfs(
            node: usize,
            graph: &Graph,
            visited: &mut [bool; 100],
            list: &mut Vec<usize>,
        ) {
            if visited[node] {
                return;
            }
            visited[node] = true;
            // visit all nodes from the adjacency list
            for edge in &graph.0[node] {
                if let Edge::To(child) = edge {
                    internal_dfs(*child, graph, visited, list);
                }
            }

            list.push(node);
        }

        // attempt traversal from all nodes to hit all possible disconnected subgraphs
        for node in 0..100 {
            if !self.0[node].is_empty() && !visited[node] {
                internal_dfs(node, self, &mut visited, &mut list);
            }
        }
        // first visited would be at the tail of the list
        list.reverse();
        list
    }
}

fn nodes(line: &str) -> Vec<usize> {
    line.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|node| node.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn solve_part_1(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let mut lines = contents.lines();

    // Assumption: only nodes 0-99 exist
    let graph = Graph::create_graph(&mut lines);

    let mut sum = 0;

    for update in lines {
        let nodes = nodes(update);
        let mut check = true;
        for (index, node) in nodes.iter().enumerate() {
            let (left_sub, right_sub) = (&nodes[..index], &nodes[index + 1..]);
            if graph.get_to_edges(*node).any(|x| left_sub.contains(&x))
                || graph.get_from_edges(*node).any(|x| right_sub.contains(&x))
            {
                check = false;
                break;
            }
        }
        if check {
            sum += nodes[nodes.len() / 2];
        }
    }

    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 5 :: Solving part 1 for {input_type} sum:\t\t{sum}");
}

pub fn solve_part_2(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("error reading the input file {file_name}");
    let mut lines = contents.lines();

    // Assumption: only nodes 0-99 exist
    let graph = Graph::create_graph(&mut lines);

    let mut sum = 0;

    for update in lines {
        let nodes = nodes(update);
        let mut check = true;
        for (index, node) in nodes.iter().enumerate() {
            let (left_sub, right_sub) = (&nodes[..index], &nodes[index + 1..]);
            if graph.get_to_edges(*node).any(|x| left_sub.contains(&x))
                || graph.get_from_edges(*node).any(|x| right_sub.contains(&x))
            {
                check = false;
                break;
            }
        }
        if !check {
            // the required wrong list
            // fix it and then add to the sum
            let mut unsorted = nodes.clone();
            unsorted.sort_by(|&a, &b| {
                if graph.get_to_edges(a).collect::<Vec<usize>>().contains(&b) {
                    Ordering::Less
                } else if graph.get_from_edges(a).collect::<Vec<usize>>().contains(&b) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            sum += unsorted[unsorted.len() / 2];
        }
    }

    let input_type = file_name.rsplit('/').next().unwrap();
    println!("Day 5 :: Solving part 2 for {input_type} sum:\t\t{sum}");
}
