use std::collections::HashMap;
use std::slice::Iter;
use std::fmt;

#[derive(Debug)]
struct Input {
    size_x: usize,
    size_y: usize,
    map: Vec<Vec<usize>>
}

#[derive(Debug)]
#[derive(Clone)]
enum Direction {Up, Down, Right, Left}

#[derive(Debug)]
#[derive(PartialEq)]
struct Point {x: isize, y: isize}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Right, Direction::Left];
        DIRECTIONS.iter()
    }
    fn delta(&self) -> Point {
        match self  {
            Self::Up => Point { x: 0, y: -1 },
            Self::Down => Point { x: 0, y: 1 },
            Self::Left => Point { x: -1, y: 0 },
            Self::Right => Point { x: 1, y: 0 },
        }
    }
    fn next_point(&self, pt: &Point) -> Point {
        let d = self.delta();
        Point { x: pt.x + d.x, y: pt.y + d.y }
    }
    fn reverse(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug)]
struct VisitData {
    cost: usize,
    from: Direction
}

#[derive(Debug)]
struct MapNode {
    cost: usize,
    visit: Option<VisitData>
}

struct Map {
    size_x: usize,
    size_y: usize,
    map: Vec<MapNode>
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size_y as isize{
            for x in 0..self.size_x as isize {
                let node = self.get_at(&Point {x, y}).unwrap();
                write!(f, "[{:1} {:2}]", node.cost, node.visit.as_ref().map(|v| v.cost).unwrap_or(0))?;
                //write!(f, "{}", node.cost)?;
            }
            writeln!(f, "");
        }
        write!(f, "")
    }
}

impl Map {
    fn from_input(input: &Input) -> Self {
        let mut map = Vec::new();
        let size_x = input.size_x;
        let size_y = input.size_y;
        for y in 0..size_y {
            let input_line = &input.map[y];
            for x in 0..size_x {
                map.push(MapNode { cost: input_line[x], visit: None });
            }
        }
        Map { size_x, size_y, map }
    }

    fn get_at(&self, pt: &Point) -> Option<&MapNode> {
        if pt.x >= 0 && pt.y >= 0 {
            let x = pt.x as usize;
            let y = pt.y as usize;
            if x < self.size_x && y < self.size_y {
                return Some(&self.map[x + y * self.size_x]);
            }
        }
        None
    }

    fn set_visit_at(&mut self, pt: &Point, cost: usize, from: Direction) {
        if pt.x >= 0 && pt.y >= 0 {
            let x = pt.x as usize;
            let y = pt.y as usize;
            if x < self.size_x && y < self.size_y {
                let node = &mut self.map[x + y * self.size_x];
                return match &mut node.visit {
                    Some(visit_data) => {
                        visit_data.cost = cost;
                        visit_data.from = from;
                    }
                    None => {
                        node.visit = Some(VisitData { cost, from });
                    }
                }
            }
        }
        panic!("bad coordinates {:?} size {} {}", pt, self.size_x, self.size_y);
    }

    fn get_visit_at(&self, pt: &Point) -> Option<&VisitData> {
        self.get_at(pt).map(|node| node.visit.as_ref()).flatten()
    }

    fn get_visit_cost_at(&self, pt: &Point) -> Option<usize> {
        self.get_visit_at(pt).map(|visit| visit.cost)
    }

    fn get_cost_at(&self, pt: &Point) -> Option<usize> {
        self.get_at(pt).map(|node| node.cost)
    }
}

fn solve_pt1(input: &Input) -> usize {

    let mut map = Map::from_input(input);
    let start = Point { x: 0, y: 0 };
    let finish = Point { x: input.size_x as isize - 1, y: input.size_y as isize - 1 };
    map.set_visit_at(&start, 0, Direction::Up);
    let mut queue = vec![Point { x: start.x, y: start.y }];

    let mut i = 0;

    while let Some(pt) = queue.pop() {

        i += 1;
        if i > 1000000 {
            println!("{:?}", map);
            break;
        }
        let cost = map.get_visit_cost_at(&pt).unwrap();
        if pt == finish {
            break;
        }
        for direction in Direction::iterator() {
            let pt1 = direction.next_point(&pt);
            if let Some(cost_at1) = map.get_cost_at(&pt1) {
                let cost1 = cost + cost_at1;
                let is_better = map.get_visit_cost_at(&pt1).map(|existing_cost1| cost1 < existing_cost1).unwrap_or(true);
                if is_better {
                    map.set_visit_at(&pt1, cost1, direction.reverse());
                    queue.push(pt1);
                }
            }
        }

        // TODO: need sort in right order
        queue.sort_by_key(|pt| map.get_visit_cost_at(pt).unwrap());
        queue.reverse();
    }

    map.get_visit_cost_at(&finish).unwrap()
}

fn solve_pt2(input: &Input) -> usize {

    let n = 5;
    let size_x = input.size_x * n;
    let size_y = input.size_y * n;
    let mut map = vec![vec![0; size_x]; size_y];
    for y in 0..size_y {
        let line = &mut map[y];
        let ny = y / input.size_y;
        let iy = y - ny * input.size_y;
        for x in 0..size_x {
            
            let nx = x / input.size_x;
            let ix = x - nx * input.size_x;
            line[x] = ((input.map[iy][ix] + nx + ny) - 1) % 9 + 1;
        }
    }
    solve_pt1(&Input {map, size_x, size_y})
}

fn test(input: &Input) {
    assert_eq!(solve_pt1(input), 40);
    
    assert_eq!(solve_pt2(input), 315);
}

pub fn main() {
    let input = read_input("input/day15.txt");
    test(input.get("test").unwrap());
    println!("day15 pt1 {}", solve_pt1(input.get("day15").unwrap()));
    println!("day15 pt2 {}", solve_pt2(input.get("day15").unwrap()));
}

fn read_input(filename: &str) -> HashMap<String, Input> {
    let mut result = HashMap::new();
    let s = std::fs::read_to_string(filename).unwrap();
    let mut lines = s.split("\n").map(|s| s.trim());
    while let Some(name) = lines.next() {
        let mut map = Vec::new();
        let mut size_x = None;
        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            let map_line = line
                .chars()
                .map(|c| c.to_string().parse())
                .flatten()
                .collect::<Vec<_>>();
            let line_size_x = map_line.len();
            match size_x {
                None => size_x = Some(line_size_x),
                Some(size_x) => assert_eq!(size_x, line_size_x)
            }
            map.push(map_line);
        }
        let size_x = size_x.unwrap();
        let size_y = map.len();
        result.insert(name.to_string(), Input { map, size_x, size_y });
    }
    result
}
