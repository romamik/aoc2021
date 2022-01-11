use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeType {
    Empty,
    CucuEast,
    CucuSouth,
}

impl NodeType {
    fn to_string(&self) -> &'static str {
        match self {
            NodeType::Empty => ".",
            NodeType::CucuEast => ">",
            NodeType::CucuSouth => "v",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node(usize);

impl Node {
    fn new() -> Node {
        Node(0)
    }

    fn set_type(&mut self, typ: NodeType) {
        let bits = match typ {
            NodeType::Empty => 0b00,
            NodeType::CucuEast => 0b01,
            NodeType::CucuSouth => 0b10,
        };
        self.0 = (self.0 & !0b11) | bits;
    }

    fn get_type(&self) -> NodeType {
        match self.0 & 0b11 {
            0b00 => NodeType::Empty,
            0b01 => NodeType::CucuEast,
            0b10 => NodeType::CucuSouth,
            _ => unreachable!(),
        }
    }

    fn set_generation(&mut self, generation: usize) {
        assert_eq!(
            (generation << 2) >> 2,
            generation,
            "generation is too big {}",
            generation
        );
        self.0 = (self.0 & 0b11) | (generation << 2);
    }

    fn get_generation(&self) -> usize {
        self.0 >> 2
    }

    fn test() {
        let mut n = Node::new();
        assert_eq!(n.get_type(), NodeType::Empty);
        assert_eq!(n.get_generation(), 0);

        n.set_type(NodeType::CucuEast);
        assert_eq!(n.get_type(), NodeType::CucuEast);

        n.set_type(NodeType::CucuSouth);
        assert_eq!(n.get_type(), NodeType::CucuSouth);

        n.set_type(NodeType::Empty);
        assert_eq!(n.get_type(), NodeType::Empty);

        n.set_generation(100);
        assert_eq!(n.get_generation(), 100);
        assert_eq!(n.get_type(), NodeType::Empty);

        let max_gen = !0 >> 2;
        n.set_generation(max_gen);
        assert_eq!(n.get_generation(), max_gen);
        assert_eq!(n.get_type(), NodeType::Empty);
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}[{}]",
            self.get_type().to_string(),
            self.get_generation()
        )
    }
}

#[derive(Clone)]
struct State {
    size_x: usize,
    size_y: usize,
    generation: usize,
    data: Vec<Node>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        if self.size_x != other.size_x || self.size_y != other.size_y {
            return false;
        }
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if self.read_current(x, y) != other.read_current(x, y) {
                    return false;
                }
            }
        }

        true
    }
}

impl State {
    fn parse(lines: &[&str]) -> State {
        let size_y = lines.len();
        let size_x = if size_y > 0 { lines[0].len() } else { 0 };
        let data = vec![Node::new(); size_x * size_y];
        let mut state = State {
            size_x,
            size_y,
            generation: 0,
            data,
        };
        lines.iter().enumerate().for_each(|(y, &line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                state.set_next(
                    x,
                    y,
                    match c {
                        '.' => NodeType::Empty,
                        '>' => NodeType::CucuEast,
                        'v' => NodeType::CucuSouth,
                        _ => panic!("unexpected char at line {} pos {}", y, x),
                    },
                );
            })
        });
        state.inc_generation();
        state
    }

    fn read_current(&self, x: usize, y: usize) -> NodeType {
        let node = self.data[x + y * self.size_x];
        if node.get_generation() == self.generation {
            node.get_type()
        } else {
            NodeType::Empty
        }
    }

    fn is_empty_current_and_next(&self, x: usize, y: usize) -> bool {
        let node = self.data[x + y * self.size_x];
        if node.get_type() == NodeType::Empty || node.get_generation() < self.generation {
            true
        } else {
            false
        }
    }

    fn set_next(&mut self, x: usize, y: usize, typ: NodeType) {
        let node = &mut self.data[x + y * self.size_x];
        node.set_generation(self.generation + 1);
        node.set_type(typ);
    }

    fn inc_generation(&mut self) {
        self.generation += 1;
    }

    fn step(&mut self) -> bool {
        let mut has_moved: bool = false;
        for (move_typ, dx, dy) in [(NodeType::CucuEast, 1, 0), (NodeType::CucuSouth, 0, 1)] {
            for y in 0..self.size_y {
                let move_y = (y + dy) % self.size_y;
                for x in 0..self.size_x {
                    let move_x = (x + dx) % self.size_x;
                    let typ = self.read_current(x, y);
                    if typ != NodeType::Empty {
                        let (new_x, new_y) =
                            if typ == move_typ && self.is_empty_current_and_next(move_x, move_y) {
                                has_moved = true;
                                (move_x, move_y)
                            } else {
                                (x, y)
                            };
                        self.set_next(new_x, new_y, typ);
                    }
                }
            }
            self.inc_generation();
        }
        has_moved
    }

    fn step_until_no_move(&mut self) -> usize {
        let mut i = 1;
        while self.step() {
            i += 1;
        }
        i
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                write!(f, "{}", self.read_current(x, y).to_string())?
            }
            writeln!(f, "")?
        }
        std::fmt::Result::Ok(())
    }
}

pub fn main() {
    let input = parse_input(&std::fs::read_to_string("input/day25.txt").unwrap());
    Node::test();
    test0(&input);
    test1(&input);
    println!("tests ok");

    println!("day 25 pt1 {}", input.get("day25").unwrap().clone().step_until_no_move());
}

fn test0(input: &HashMap<String, State>) {
    let mut state = input.get("test0").unwrap().clone();
    for _ in 0..4 {
        state.step();
    }
    assert_eq!(&state, input.get("test0_after_4").unwrap());
}

fn test1(input: &HashMap<String, State>) {
    let mut state = input.get("test1").unwrap().clone();
    for step in 0..57 {
        let moved = state.step();
        assert_eq!(moved, true, "moved at step {}", step);
    }
    let moved = state.step();
    assert_eq!(moved, false);
    assert_eq!(&state, input.get("test1_after_58").unwrap());

    let mut state = input.get("test1").unwrap().clone();
    let steps = state.step_until_no_move();
    assert_eq!(steps, 58);
}

fn parse_input(s: &str) -> HashMap<String, State> {
    s.split("\n\n")
        .map(|s| {
            let lines = s.lines().collect::<Vec<_>>();
            let name = lines[0].to_string();
            let state = State::parse(&lines[1..]);
            (name, state)
        })
        .collect()
}
