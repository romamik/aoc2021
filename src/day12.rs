use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Graph (HashMap::<String, Vec::<String>>);

static EMPTY: Vec::<String> = vec![];
const START: &str = "start";
const FINISH: &str = "end";

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

struct Path<'a> {last: &'a str, has_2_lowercase: bool, prev_path: Option<Rc::<Path<'a>>>}

#[allow(dead_code)]
impl Path<'_> {
    fn to_vector(&self) -> Vec::<String> {

        fn visit(path: &Path, result: &mut Vec::<String>) {
            match &path.prev_path {
                None => (),
                Some(p) => visit(p, result)
            }
            result.push(path.last.to_string());
        }

        let mut result = Vec::new();
        visit(self, &mut result);
        result
    }
}

fn find_all_paths<'a, F>(graph: &'a Graph, from: &'a str, to: &'a str, mut is_last_vert_valid: F) -> Vec::<Rc::<Path<'a>>>
where F: FnMut(&mut Path) -> bool {

    let init_path = Rc::new(Path {last: from, has_2_lowercase: false, prev_path: None});
    let mut queue = vec![init_path];
    let mut found_paths = vec![];
    while queue.len() > 0 {
        let path = queue.pop().unwrap();
        let last = path.last;
        if last == to {
            found_paths.push(path);
        }
        else {
            
            for next_vert in graph.get_edges_from(last).iter() {
                let mut new_path = Path {last: next_vert, has_2_lowercase: path.has_2_lowercase, prev_path: Some(path.clone())};
                if is_last_vert_valid(&mut new_path) {            
                    queue.push(Rc::new(new_path));
                }
            }
        }
    }

    found_paths
}

fn solve_pt1(graph: &Graph) -> usize {

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
    
    find_all_paths(graph, START, FINISH, |path| {
        can_visit_multiple(path.last) || !is_vert_in_path(path.last, &path.prev_path)
    }).len()
}

fn solve_pt2(graph: &Graph) -> usize {
    
    fn count_vert_in_path(vert: &str, path: &Option<Rc::<Path>>) -> usize {
        match path {
            None => 0,
            Some(path) => {
                let last_count = if path.last == vert { 1 } else { 0 };
                last_count + count_vert_in_path(vert, &path.prev_path)
            }
        }
    }

    fn is_last_vert_valid(path: &mut Path) -> bool {
        let last = path.last;
        if last == START {
            false
        }
        else if last == FINISH {
            true
        }
        else {
            let is_upper = last.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);
            if is_upper {
                true
            }
            else {
                let prev_count = count_vert_in_path(last, &path.prev_path);
                if prev_count == 0 {
                    true
                }
                else if prev_count == 1 && !path.has_2_lowercase {
                    path.has_2_lowercase = true;
                    true
                }
                else {
                    false
                }
            }
        }
    }

    let paths = find_all_paths(graph, START, FINISH, is_last_vert_valid);
    //println!("{:#?}", paths.iter().map(|p| p.to_vector().join(",")).collect::<Vec<_>>());
    paths.len()
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

    println!("test1 pt1 {}", solve_pt1(&test_input1));
    println!("test2 pt1 {}", solve_pt1(&test_input2));
    println!("test3 pt1 {}", solve_pt1(&test_input3));
    println!("day12 pt1 {}", solve_pt1(&day12_input));
    println!("test1 pt2 {}", solve_pt2(&test_input1));
    println!("test2 pt2 {}", solve_pt2(&test_input2));
    println!("test3 pt2 {}", solve_pt2(&test_input3));
    println!("day12 pt2 {}", solve_pt2(&day12_input));
}

fn parse_input(s: &str) -> Graph {
    let mut graph: Graph = Graph::new();
    s.split("\n").map(|s| s.trim()).filter(|s| s.len() > 0).for_each(|s| {
        let arr = <[&str; 2]>::try_from(s.split("-").collect::<Vec<_>>()).unwrap();
        graph.add_edge(arr[0], arr[1]);
    });
    graph
}