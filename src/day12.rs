use std::collections::HashMap;

#[derive(Debug)]
struct Graph (HashMap::<String, Vec::<String>>);

static empty: Vec::<String> = vec![];

impl Graph {

    fn new() -> Graph {
        Graph (HashMap::new())
    }

    fn get_edges_from(&self, vert: &str) -> &[String] {
        match self.0.get(vert) {
            Some(vec) => vec,
            None => &empty
        }
    }

    fn add_edge(&mut self, vert0: &str, vert1: &str) {
        self.add_one_way_edge(vert0, vert1);
        self.add_one_way_edge(vert1, vert0);
    }

    fn add_one_way_edge(&mut self, vert0: &str, vert1: &str) {
        match self.0.get_mut(vert0) {
            Some(vec) => vec.push(vert1.to_string()),
            None => {self.0.insert(vert0.to_string(), vec![vert1.to_string()]);},
        }
    }
}

fn can_visit_multiple(vert: &str) -> bool {
    vert.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn is_vert_in_path(vert: &str, path: &Vec<&str>) -> bool {
    path.iter().any(|&v| v == vert)
}

// todo - use linked lists for path to avoid clone()
fn find_all_paths<'a>(graph: &'a Graph, from: &'a str, to: &'a str) -> Vec::<Vec::<&'a str>> {

    let mut paths = vec![];
    let mut queue = vec![vec![from]];
    while queue.len() > 0 {
        let prev_path = queue.pop().unwrap();
        let last_vert = prev_path.last().unwrap();
        for next_vert in graph.get_edges_from(last_vert).iter() {
            if next_vert == to {
                paths.push(prev_path.clone());
            }
            else if can_visit_multiple(next_vert) || !is_vert_in_path(next_vert, &prev_path) {
                let mut new_path = prev_path.clone();
                new_path.push(next_vert);
                queue.push(new_path);
            }
        }
    }

    paths
}


pub fn main() {
    let test_input1 = parse_input("
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ");
    let test_input2 = parse_input("
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ");
    let test_input3 = parse_input("
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ");
    let day12_input = parse_input(&std::fs::read_to_string("input/day12.txt").unwrap());

    println!("test1 pt1 {}", find_all_paths(&test_input1, "start", "end").iter().count());
    println!("test2 pt1 {}", find_all_paths(&test_input2, "start", "end").iter().count());
    println!("test3 pt1 {}", find_all_paths(&test_input3, "start", "end").iter().count());
    println!("day12 pt1 {}", find_all_paths(&day12_input, "start", "end").iter().count());
}

fn parse_input(s: &str) -> Graph {
    let mut graph: Graph = Graph::new();
    s.split("\n").map(|s| s.trim()).filter(|s| s.len() > 0).for_each(|s| {
        let arr = <[&str; 2]>::try_from(s.split("-").collect::<Vec<_>>()).unwrap();
        graph.add_edge(arr[0], arr[1]);
    });
    graph
}