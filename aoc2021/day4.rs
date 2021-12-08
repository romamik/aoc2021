use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

const BOARD_SIZE: usize = 5;
type BoardDef = [[u8; BOARD_SIZE]; BOARD_SIZE];
type BoardState = [[bool; BOARD_SIZE]; BOARD_SIZE];

struct BingoSet {
    numbers: Vec<u8>, 
    board_defs: Vec<BoardDef>,
}

#[derive(Debug)]
struct Board<'a> {
    def: &'a BoardDef,
    state: BoardState
}

impl Board<'_> {
    
    fn new(def: &BoardDef) -> Board {
        Board {
            def: def,
            state: [[false; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    fn number_at(&self, x: usize, y: usize) -> u8 {
        self.def[y][x]
    }

    fn marked_at(&self, x: usize, y: usize) -> bool {
        self.state[y][x]
    }

    fn set_marked_at(&mut self, x: usize, y: usize) {
        self.state[y][x] = true;
    }

    fn marked_row(&self, y: usize) -> bool {
        for x in 0..BOARD_SIZE {
            if !self.marked_at(x, y) {
                return false;
            }
        }
        true
    }

    fn marked_column(&self, x: usize) -> bool {
        for y in 0..BOARD_SIZE {
            if !self.marked_at(x, y) {
                return false;
            }
        }
        true
    }

    fn has_marked_row_or_column(&self) -> bool {
        for i in 0..BOARD_SIZE {
            if self.marked_row(i) || self.marked_column(i) {
                return true;
            }
        }
        false
    }

    fn calc_unmarked_sum(&self) -> u32 {
        let mut sum = 0_u32;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if !self.marked_at(x, y) {
                    sum += self.number_at(x, y) as u32;
                }
            }
        }
        sum
    }

    fn add_number(&mut self, number: u8) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.number_at(x, y) == number {
                    self.set_marked_at(x, y);
                }
            }
        }
    }
}

fn create_boards(set: &BingoSet) -> Vec<Board> {

    let mut boards = Vec::new();
    for board_def in set.board_defs.iter() {
        boards.push(Board::new(board_def));
    }
    boards
}

fn find_first_winning_board(set: &BingoSet) -> Option<(u8, Board)> {

    let mut boards = create_boards(set);

    for number in set.numbers.iter() {
        for board in boards.iter_mut() {
            board.add_number(*number);
        }
        let pos = boards.iter().position(|board| board.has_marked_row_or_column());
        if let Some(pos) = pos {
            return Some((*number, boards.swap_remove(pos)));
        }
    }
    None
}

fn find_last_winning_board(set: &BingoSet) -> Option<(u8, Board)> {

    let mut boards = create_boards(set);
    let mut last_winning_board_and_number = None;
    for number in set.numbers.iter() {
        for board in boards.iter_mut() {
            board.add_number(*number);
        }
        while let Some(pos) = boards.iter().position(|board| board.has_marked_row_or_column()) {
            let board = boards.swap_remove(pos);
            //println!("{}, {}, {:?}", pos, number, board);
            last_winning_board_and_number = Some((*number, board));
        }
    }
    last_winning_board_and_number
}

fn solve_pt1(set: &BingoSet) -> Option<u32> {

    match find_first_winning_board(set) {
        Some((number, board)) => Some(board.calc_unmarked_sum() * number as u32),
        None => None
    }
}

fn solve_pt2(set: &BingoSet) -> Option<u32> {

    match find_last_winning_board(set) {
        Some((number, board)) => Some(board.calc_unmarked_sum() * number as u32),
        None => None
    }
}


fn main() {

    let test_set = BingoSet {
        numbers: vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
        board_defs: vec![
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ],
            [
                [3, 15,  0,  2, 22],
                [9, 18, 13, 17,  5],
                [19,  8,  7, 25, 23],
                [20, 11, 10, 24,  4],
                [14, 21, 16, 12,  6],
            ],
            [
                [14, 21, 17, 24,  4],
                [10, 16, 15,  9, 19],
                [18,  8, 23, 26, 20],
                [22, 11, 13,  6,  5],
                [2,  0, 12,  3,  7],
            ]
        ],
    };

    let day4_set = read_bingo_set("day4.txt");

    println!("test pt1 {:?}", solve_pt1(&test_set));
    println!("day4 pt1 {:?}", solve_pt1(&day4_set));
    println!("test pt2 {:?}", solve_pt2(&test_set));
    println!("day4 pt2 {:?}", solve_pt2(&day4_set));
}

fn read_bingo_set<P>(filename: P) -> BingoSet
where P: AsRef<Path> {

    let file = File::open(filename).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|v| v.unwrap()).collect();
    let numbers: Vec<u8> = lines[0].split(',').map(|s| s.parse::<u8>().unwrap()).collect();
    
    let num_boards = (lines.len() - 1) / (BOARD_SIZE + 1);
    let mut board_defs = Vec::new();
    for board_n in 0..num_boards {
        let i = board_n * (BOARD_SIZE + 1) + 2;
        let mut rows = Vec::new();
        for line in &lines[i..i + BOARD_SIZE] {
            let row: [u8; BOARD_SIZE] = line.split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap();
            rows.push(row);
        }
        let board_def: BoardDef = rows.try_into().unwrap();
        board_defs.push(board_def);
    }

    BingoSet {
        numbers: numbers,
        board_defs: board_defs,
    }
}