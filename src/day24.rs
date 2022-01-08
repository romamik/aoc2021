use std::collections::HashMap;

type Register = usize;

fn register_by_name(name: &str) -> Register {
    match name {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!("bad register name {}", name),
    }
}

#[derive(Debug)]
enum Arg {
    Register(Register),
    Value(isize),
}

fn parse_arg(s: &str) -> Arg {
    if let Ok(val) = s.parse::<isize>() {
        Arg::Value(val)
    } else {
        Arg::Register(register_by_name(s))
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
        ["inp", r] => Command::Inp(register_by_name(r)),
        ["add", r, a] => Command::Add(register_by_name(r), parse_arg(a)),
        ["mul", r, a] => Command::Mul(register_by_name(r), parse_arg(a)),
        ["div", r, a] => Command::Div(register_by_name(r), parse_arg(a)),
        ["mod", r, a] => Command::Mod(register_by_name(r), parse_arg(a)),
        ["eql", r, a] => Command::Eql(register_by_name(r), parse_arg(a)),
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

type State = [isize; 4];

fn get_arg_value(state: &State, arg: &Arg) -> isize {
    match arg {
        Arg::Value(v) => *v,
        Arg::Register(r) => state[*r],
    }
}

fn run_program(program: &[Command], input: &[isize]) -> State {
    let mut state: State = [0; 4];
    let mut input_pos = 0;
    for command in program {
        match command {
            Command::Inp(r) => {
                state[*r] = input[input_pos];
                input_pos += 1;
            }
            Command::Add(r, a) => state[*r] = state[*r] + get_arg_value(&state, a),
            Command::Mul(r, a) => state[*r] = state[*r] * get_arg_value(&state, a),
            Command::Div(r, a) => state[*r] = state[*r] / get_arg_value(&state, a),
            Command::Mod(r, a) => state[*r] = state[*r] % get_arg_value(&state, a),
            Command::Eql(r, a) => {
                state[*r] = if state[*r] == get_arg_value(&state, a) {
                    1
                } else {
                    0
                }
            }
        }
    }
    state
}

fn read_state(state: &State, register_name: &str) -> isize {
    state[register_by_name(register_name)]
}

fn find_max_model_number(program: &[Command]) -> usize {
    fn dec_num(num: &mut [isize]) {
        let mut p = num.len() - 1;
        loop {
            if num[p] > 1 {
                num[p] -= 1;
                break;
            } else {
                num[p] = 9;
                p -= 1;
            }
        }
    }
    
    const MODEL_NUMBER_LEN: usize = 14;
    let mut num = std::iter::repeat(9)
        .take(MODEL_NUMBER_LEN)
        .collect::<Vec<isize>>();
    loop {
        let z = read_state(&run_program(program, &num), "z");
        //println!("{:?} {:?}", z, num);
        if z == 0 {
            break;
        }
        dec_num(&mut num);
    }
    num.iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn main() {
    let input = parse_input(&std::fs::read_to_string("input/day24.txt").unwrap());
    assert_eq!(
        read_state(
            &run_program(input.get("negate").unwrap(), &mut vec![10]),
            "x"
        ),
        -10
    );
    assert_eq!(
        read_state(
            &run_program(input.get("is_3_times_bigger").unwrap(), &mut vec![10, 30]),
            "z"
        ),
        1
    );
    assert_eq!(
        read_state(
            &run_program(input.get("is_3_times_bigger").unwrap(), &mut vec![10, 31]),
            "z"
        ),
        0
    );
    assert_eq!(
        run_program(input.get("get_bits").unwrap(), &mut vec![0b1010]),
        [1, 0, 1, 0]
    );
    assert_eq!(
        run_program(input.get("get_bits").unwrap(), &mut vec![0b0101]),
        [0, 1, 0, 1]
    );
    println!("tests ok");
    println!(
        "day 24 pt1 {}",
        find_max_model_number(input.get("day24").unwrap())
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
