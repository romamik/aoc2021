use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Graph (HashMap::<String, Vec::<String>>);

static EMPTY: Vec::<String> = vec![];

impl Graph {

    fn new() -> Graph {
        Graph (HashMap::new())
    }

    fn get_edges_from(&self, vert: &str) -> &[String] {
        match self.0.get(vert) {
            Some(vec) => vec,
            None => &EMPTY
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

struct Path<'a> {last: &'a str, prev_path: Option<Rc::<Path<'a>>>}

fn can_visit_multiple(vert: &str) -> bool {
    vert.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

fn is_vert_in_path(vert: &str, path: &Option<Rc::<Path>>) -> bool {
    match path {
        None => false,
        Some(path) if path.last == vert => true,
        Some(path) => is_vert_in_path(vert, &path.prev_path),
    }
}

fn find_all_paths<'a>(graph: &'a Graph, from: &'a str, to: &'a str) -> usize {

    let init_path = Rc::new(Path {last: from, prev_path: None});
    let mut queue = vec![init_path];
    let mut found_paths = vec![];
    while queue.len() > 0 {
        let path = queue.pop().unwrap();
        let last = path.last;
        if last == to {
            found_paths.push(path);
        }
        else if can_visit_multiple(last) || !is_vert_in_path(last, &path.prev_path) {
            for next_vert in graph.get_edges_from(last).iter() {
                let new_path = Rc::new(Path {last: next_vert, prev_path: Some(path.clone())});
                queue.push(new_path);
            }
        }
    }

    found_paths.len()
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

    println!("test1 pt1 {}", find_all_paths(&test_input1, "start", "end"));
    println!("test2 pt1 {}", find_all_paths(&test_input2, "start", "end"));
    println!("test3 pt1 {}", find_all_paths(&test_input3, "start", "end"));
    println!("day12 pt1 {}", find_all_paths(&day12_input, "start", "end"));
}

fn parse_input(s: &str) -> Graph {
    let mut graph: Graph = Graph::new();
    s.split("\n").map(|s| s.trim()).filter(|s| s.len() > 0).for_each(|s| {
        let arr = <[&str; 2]>::try_from(s.split("-").collect::<Vec<_>>()).unwrap();
        graph.add_edge(arr[0], arr[1]);
    });
    graph
}