use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    val: i32
}

fn main() -> Result<()> {
    
    let test_lines = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2"
    ];

    let test_commands = lines2commands(&test_lines)?;
    let day2_commands = read_commands("day2.txt")?;

    let (x, depth) = find_position(&test_commands);
    println!("test {0}*{1}={2}", x, depth, x * depth);

    let (x, depth) = find_position_pt2(&test_commands);
    println!("test pt2 {0}*{1}={2}", x, depth, x * depth);

    let (x, depth) = find_position(&day2_commands);
    println!("day2 {0}*{1}={2}", x, depth, x * depth);

    let (x, depth) = find_position_pt2(&day2_commands);
    println!("day2 pt2 {0}*{1}={2}", x, depth, x * depth);

    Ok(())
}

fn find_position_pt2(commands: &[Command]) -> (i32, i32) {
    let mut x = 0_i32;
    let mut depth = 0_i32;
    let mut aim = 0_i32;

    for command in commands.iter() {
        match &command.dir {
            Direction::Forward => {
                x += command.val;
                depth += command.val * aim;
            },
            Direction::Down => aim += command.val,
            Direction::Up => aim -= command.val
        }
    }

    (x, depth)
}

fn find_position(commands: &[Command]) -> (i32, i32) {
    
    let mut x = 0_i32;
    let mut depth = 0_i32;
    for command in commands.iter() {
        match &command.dir {
            Direction::Forward => x += command.val,
            Direction::Down => depth += command.val,
            Direction::Up => depth -= command.val
        }
    }

    (x, depth)
}

fn lines2commands(lines: &[&str]) -> Result<Vec<Command>> {

    let mut commands = Vec::new();
    for line in lines.iter() {
        commands.push(Command::from_str(line)?);
    }
    Ok(commands)
}

#[derive(Debug)]
struct UnknownDirectionErr;
impl error::Error for UnknownDirectionErr {}
impl fmt::Display for UnknownDirectionErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnknownDirectionErr")
    }
}

impl FromStr for Direction {

    type Err = Box<dyn error::Error>;

    fn from_str(input: &str) -> Result<Direction> {
        match &*input.to_lowercase() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _  => Err(UnknownDirectionErr.into()),
        }
    }
}

#[derive(Debug)]
struct BadInputStringErr;
impl error::Error for BadInputStringErr {}
impl fmt::Display for BadInputStringErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BadInputStringErr")
    }
}

impl FromStr for Command {

    type Err = Box<dyn error::Error>;

    fn from_str(input: &str) -> Result<Command> {
        
        let split = input.split(" ").collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(BadInputStringErr.into());
        }
        let t: &str = split[1];
        let t: i32 = t.parse()?;
        Ok(Command {
            dir: Direction::from_str(split[0])?,
            val: t
        })
    }
}

fn read_commands<P>(filename: P) -> Result<Vec::<Command>> 
where P: AsRef<Path> {

    let mut vec = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines {
        vec.push(Command::from_str(&line?)?);
    }
    Ok(vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}