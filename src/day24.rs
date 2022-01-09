/*
We have an interpreter of ALU instructions that operate Ranges
To find maximum number we start with number consisting of digits, where each digit is a range [1..9]
Then we try ranges [1..1], [2..2] .. [9..9] for every digit in number, but continue to next digit only if resulting range for z includes 0.
*/
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Register(usize);

impl Register {
    fn from_name(name: &str) -> Self {
        match name {
            "w" => Self(0),
            "x" => Self(1),
            "y" => Self(2),
            "z" => Self(3),
            _ => panic!("bad register name {}", name),
        }
    }
}

#[derive(Debug)]
enum Arg {
    Register(Register),
    Value(isize),
}

impl Arg {
    fn parse(s: &str) -> Self {
        if let Ok(val) = s.parse::<isize>() {
            Self::Value(val)
        } else {
            Self::Register(Register::from_name(s))
        }
    }
}

#[derive(PartialEq, Debug)]
struct State([Range; 4]);

impl State {
    fn new() -> State {
        State([Range::new(0, 0); 4])
    }

    fn get_register(&self, register: Register) -> &Range {
        &self.0[register.0]
    }

    fn get_register_by_name(&self, name: &str) -> &Range {
        self.get_register(Register::from_name(name))
    }

    fn set_register(&mut self, register: Register, value: Range) {
        self.0[register.0] = value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    from: isize,
    to: isize,
}

impl Range {
    fn new(a: isize, b: isize) -> Range {
        if a <= b {
            Range { from: a, to: b }
        } else {
            Range { from: b, to: a }
        }
    }

    fn from_multi_values(values: &[isize]) -> Range {
        Range::new(*values.iter().min().unwrap(), *values.iter().max().unwrap())
    }

    fn add_range(&self, rhs: &Self) -> Self {
        Range::new(self.from + rhs.from, self.to + rhs.to)
    }

    fn add_value(&self, rhs: isize) -> Self {
        Range::new(self.from + rhs, self.to + rhs)
    }

    fn add(&self, state: &State, rhs: &Arg) -> Self {
        match rhs {
            Arg::Register(r) => self.add_range(state.get_register(*r)),
            Arg::Value(v) => self.add_value(*v),
        }
    }

    fn mul_range(&self, rhs: &Self) -> Self {
        Range::from_multi_values(&[
            self.from * rhs.from,
            self.from * rhs.to,
            self.to * rhs.from,
            self.to * rhs.to,
        ])
    }

    fn mul_value(&self, rhs: isize) -> Self {
        Range::new(self.from * rhs, self.to * rhs)
    }

    fn mul(&self, state: &State, rhs: &Arg) -> Self {
        match rhs {
            Arg::Register(r) => self.mul_range(state.get_register(*r)),
            Arg::Value(v) => self.mul_value(*v),
        }
    }

    fn div_value(&self, rhs: isize) -> Self {
        Range::new(self.from / rhs, self.to / rhs)
    }

    fn div(&self, _state: &State, rhs: &Arg) -> Self {
        match rhs {
            Arg::Register(_) => panic!("div by register not implemented"),
            Arg::Value(v) => self.div_value(*v),
        }
    }

    fn mod_value(&self, rhs: isize) -> Self {
        assert!(rhs > 0);

        fn mod_positive_range(a: isize, b: isize, rhs: isize) -> Range {
            if b - a < rhs {
                // just process all values
                // it is possible to specify range explicitly with some if statements but thats not done for now
                Range::from_multi_values(&(a..=b).map(|v| v % rhs).collect::<Vec<_>>())
            } else {
                Range::new(0, rhs - 1)
            }
        }

        if self.from >= 0 && self.to >= 0 {
            mod_positive_range(self.from, self.to, rhs)
        } else if self.from <= 0 && self.to <= 0 {
            mod_positive_range(-self.to, -self.from, rhs).mul_value(-1)
        } else {
            // different signs and thus `from <= 0` and `to >= 0`
            Range::new(
                std::cmp::max(self.from, -rhs + 1),
                std::cmp::min(self.to, rhs - 1),
            )
        }
    }

    fn modulo(&self, _state: &State, rhs: &Arg) -> Self {
        match rhs {
            Arg::Register(_) => panic!("mod by register not implemented"),
            Arg::Value(v) => self.mod_value(*v),
        }
    }

    fn eql_range(&self, rhs: &Self) -> Self {
        // eql return 0 if values are not equal and 1 if values are equal
        if rhs.from == rhs.to {
            self.eql_value(rhs.from)
        } else if self.from == self.to {
            rhs.eql_value(self.from)
        } else if self.to < rhs.from || self.from > rhs.to {
            // ranges does not intersect, result is always 0
            Range::new(0, 0)
        } else {
            // ranges intersect, that means that result can be both 0 or 1
            Range::new(0, 1)
        }
    }

    fn eql_value(&self, rhs: isize) -> Self {
        if self.from == rhs && self.to == rhs {
            Range::new(1, 1)
        } else if self.from > rhs || self.to < rhs {
            Range::new(0, 0)
        } else {
            Range::new(0, 1)
        }
    }

    fn eql(&self, state: &State, rhs: &Arg) -> Self {
        match rhs {
            Arg::Register(r) => self.eql_range(state.get_register(*r)),
            Arg::Value(v) => self.eql_value(*v),
        }
    }
}

#[derive(Debug)]
enum Command {
    Inp(Register),
    Add(Register, Arg),
    Mul(Register, Arg),
    Div(Register, Arg),
    Mod(Register, Arg),
    Eql(Register, Arg),
}

fn parse_command(s: &str) -> Command {
    let parts: Vec<&str> = s.split_ascii_whitespace().collect();
    match parts[..] {
        ["inp", r] => Command::Inp(Register::from_name(r)),
        ["add", r, a] => Command::Add(Register::from_name(r), Arg::parse(a)),
        ["mul", r, a] => Command::Mul(Register::from_name(r), Arg::parse(a)),
        ["div", r, a] => Command::Div(Register::from_name(r), Arg::parse(a)),
        ["mod", r, a] => Command::Mod(Register::from_name(r), Arg::parse(a)),
        ["eql", r, a] => Command::Eql(Register::from_name(r), Arg::parse(a)),
        _ => panic!("cannot parse command {}", s),
    }
}

fn parse_program(s: &str) -> Vec<Command> {
    s.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_command)
        .collect()
}

fn run_program(program: &[Command], input: &[Range]) -> State {
    let mut state: State = State::new();
    let mut input_pos = 0;
    for command in program {
        match command {
            Command::Inp(r) => {
                state.set_register(*r, input[input_pos].clone());
                input_pos += 1;
            }
            Command::Add(r, a) => state.set_register(*r, state.get_register(*r).add(&state, a)),
            Command::Mul(r, a) => state.set_register(*r, state.get_register(*r).mul(&state, a)),
            Command::Div(r, a) => state.set_register(*r, state.get_register(*r).div(&state, a)),
            Command::Mod(r, a) => state.set_register(*r, state.get_register(*r).modulo(&state, a)),
            Command::Eql(r, a) => state.set_register(*r, state.get_register(*r).eql(&state, a)),
        }
    }
    state
}

fn find_first_number_with_z_0(
    program: &[Command],
    digits: &[isize],
    num_len: usize,
) -> Option<usize> {
    fn fun(
        input: &mut [Range],
        pos: usize,
        program: &[Command],
        digits: &[isize],
        num_len: usize,
    ) -> bool {
        //println!("{:?}", input);
        if pos == num_len {
            return true;
        }
        for digit in digits.iter() {
            input[pos] = Range::new(*digit, *digit);
            let z_range = run_program(program, input)
                .get_register_by_name("z")
                .clone();
            let z_canbe_0 = z_range.from <= 0 && z_range.to >= 0;
            if z_canbe_0 {
                if fun(input, pos + 1, program, digits, num_len) {
                    return true;
                }
            }
        }
        input[pos] = Range::new(*digits.first().unwrap(), *digits.last().unwrap());
        return false;
    }

    let mut input = vec![Range::new(*digits.first().unwrap(), *digits.last().unwrap()); num_len];
    if fun(&mut input, 0, program, digits, num_len) {
        let result = input
            .iter()
            .map(|range| {
                assert_eq!(range.from, range.to);
                range.from.to_string()
            })
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let z = run_program(program, &input).get_register_by_name("z").clone();
        assert_eq!(z.from, z.to);
        assert_eq!(z.from, 0);

        Some(result)
    } else {
        None
    }
}

pub fn main() {
    let input = parse_input(&std::fs::read_to_string("input/day24.txt").unwrap());
    assert_eq!(
        run_program(input.get("negate").unwrap(), &mut vec![Range::new(10, 10)])
            .get_register_by_name("x"),
        &Range::new(-10, -10)
    );
    assert_eq!(
        run_program(
            input.get("is_3_times_bigger").unwrap(),
            &mut vec![Range::new(10, 10), Range::new(30, 30)]
        )
        .get_register_by_name("z"),
        &Range::new(1, 1)
    );
    assert_eq!(
        run_program(
            input.get("is_3_times_bigger").unwrap(),
            &mut vec![Range::new(10, 10), Range::new(31, 31)]
        )
        .get_register_by_name("z"),
        &Range::new(0, 0)
    );
    assert_eq!(
        run_program(
            input.get("get_bits").unwrap(),
            &mut vec![Range::new(0b1010, 0b1010)]
        ),
        State([
            Range::new(1, 1),
            Range::new(0, 0),
            Range::new(1, 1),
            Range::new(0, 0)
        ])
    );
    assert_eq!(
        run_program(
            input.get("get_bits").unwrap(),
            &mut vec![Range::new(0b0101, 0b0101)]
        ),
        State([
            Range::new(0, 0),
            Range::new(1, 1),
            Range::new(0, 0),
            Range::new(1, 1)
        ])
    );
    println!("tests ok");
    println!(
        "day 24 pt1 {:?}",
        find_first_number_with_z_0(
            input.get("day24").unwrap(),
            &vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            14,
        )
    );
    println!(
        "day 24 pt2 {:?}",
        find_first_number_with_z_0(
            input.get("day24").unwrap(),
            &vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            14,
        )
    );
}

fn parse_input(s: &str) -> HashMap<String, Vec<Command>> {
    s.split("\n\n")
        .map(|s| {
            let lines: Vec<_> = s.lines().collect();
            let name = lines[0].to_string();
            let program = parse_program(&lines[1..].join("\n"));
            (name, program)
        })
        .collect()
}
