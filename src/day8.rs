use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

type Digit = HashSet<u8>;
type Digits = [Digit; 10];
type InputLine = [Vec<Digit>; 2];
type Input = Vec<InputLine>;

fn solve_pt1(digits: &Digits, input: &Input) -> i32 {
    
    let mut digits_by_len: HashMap<usize, Vec<&Digit>> = HashMap::new();
    for digit in digits {
        let len = digit.len();
        match digits_by_len.get_mut(&len) {
            Some(v) => v.push(&digit),
            None => { digits_by_len.insert(len, vec![&digit]); }
        }
    }

    let single_digits = digits_by_len.iter().filter_map(|(&_len, digits)| {
        if digits.len() == 1 {
            Some(digits[0])
        }
        else {
            None
        }
    }).collect::<Vec<_>>();
    
    //println!("{:?}", digits_by_len);
    //println!("{:?}", single_digits);

    let mut count = 0;

    for line in input {
        for digit in line[1].iter() {
            if single_digits.iter().position(|d| d.len() == digit.len()).is_some() {
                count += 1;
                //println!("{:?}", digit);
            }
        }
    }

    count
}

fn solve_pt2_impl(digits: &Digits, input_line: &InputLine) -> i32 {

    let mut possible_mappings = HashMap::new();
    for i in 0..10 {
        possible_mappings.insert(i, Vec::from_iter(0..10));
    }

    for input_digit in input_line[0] {
        for digit in digits {
            if digit_can_be_mapped_to_input_digit {
                
            }
        }
    }

    0
}

fn solve_pt2(digits: &Digits, input: &Input) -> i32 {

    let mut sum = 0;
    for line in input {
        sum += solve_pt2_impl(digits, line)
    }
    sum
}

pub fn main() {

    let digits = get_digits();
    let test_input = parse_input("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");

    let day8_input = parse_input(&fs::read_to_string("input/day8.txt").unwrap());

    println!("test pt1 {}", solve_pt1(&digits, &test_input));
    println!("day8 pt1 {}", solve_pt1(&digits, &day8_input));
    println!("test pt2 {}", solve_pt2(&digits, &test_input));
}

fn get_digits() -> Digits {
//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
    [
        "abefg", //0
        "cf", // 1
        "acdeg", // 2
        "acdfg", // 3
        "bcdf", // 4
        "abdfg", // 5
        "abdefg", // 6
        "acf", // 7
        "abcdefg", // 8
        "abcdfg", // 9
    ].map(|s| HashSet::from_iter(s.as_bytes().iter().cloned()))
}

fn parse_input(s: &str) -> Input {
    s.split("\n")
    .map(|s| 
        s.split("|")
        .map(|s| 
            s.split_whitespace()
            .map(|s| HashSet::<u8>::from_iter(s.as_bytes().iter().cloned()))
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>()
        .try_into().unwrap()
    )
    .collect::<Vec<_>>()
}