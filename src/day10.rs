use std::fs;

#[derive(PartialEq, Debug)]
enum BracketType {
    Parentheses,
    Brackets,
    Braces,
    Angle,
}

enum CharType {
    Open(BracketType),
    Close(BracketType),
    NotBracket,
}

fn get_char_type(c: char) -> CharType {
    match c {
        '(' => CharType::Open(BracketType::Parentheses),
        ')' => CharType::Close(BracketType::Parentheses),
        '[' => CharType::Open(BracketType::Brackets),
        ']' => CharType::Close(BracketType::Brackets),
        '{' => CharType::Open(BracketType::Braces),
        '}' => CharType::Close(BracketType::Braces),
        '<' => CharType::Open(BracketType::Angle),
        '>' => CharType::Close(BracketType::Angle),
        _ => CharType::NotBracket,
    }
}

fn solve(input: Vec<Vec<char>>) -> (i32, i64) {
    let mut pt1_score = 0;
    let mut pt2_scores = Vec::new();

    for line in input {
        let mut open_brackets = Vec::<BracketType>::new();
        let mut bad_close_bracket_type = None;
        for c in line.iter() {
            match get_char_type(*c) {
                CharType::Open(bracket_type) => {
                    open_brackets.push(bracket_type);
                }
                CharType::Close(close_bracket_type) => {
                    let is_good_close = open_brackets
                        .last()
                        .map(|open_bracket_type| *open_bracket_type == close_bracket_type)
                        .unwrap_or(false);
                    if is_good_close {
                        open_brackets.pop();
                    } else {
                        bad_close_bracket_type = Some(close_bracket_type);
                        break;
                    }
                }
                CharType::NotBracket => {
                    panic!()
                }
            }
        }
        match bad_close_bracket_type {
            Some(bracket_type) => {
                pt1_score += match bracket_type {
                    BracketType::Parentheses => 3,
                    BracketType::Brackets => 57,
                    BracketType::Braces => 1197,
                    BracketType::Angle => 25137,
                }
            },
            None => {
                let mut line_score = 0;
                for bracket_type in open_brackets.iter().rev() {
                    line_score *= 5;
                    line_score += match bracket_type {
                        BracketType::Parentheses => 1,
                        BracketType::Brackets => 2,
                        BracketType::Braces => 3,
                        BracketType::Angle => 4,
                    }
                }
                pt2_scores.push(line_score)
            },
        }
    }

    pt2_scores.sort();
    let pt2_score = pt2_scores[pt2_scores.len() / 2];
    (pt1_score, pt2_score)
}

pub fn main() {
    let test_input = "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    let test_input = parse_input(test_input);
    let day10_input = parse_input(&fs::read_to_string("input/day10.txt").unwrap());

    println!("test {:?}", solve(test_input));
    println!("day10 {:?}", solve(day10_input));
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
