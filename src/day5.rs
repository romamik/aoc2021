use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::convert::TryInto;

fn visit(map: &mut HashMap<String, i32>, overlap_count: &mut i32, x: i32, y: i32) {
    let key = format!("{}_{}", x, y);
    let val: i32 = map.get(&key).unwrap_or(&0) + 1;
    map.insert(key, val);
    if val == 2 {
        *overlap_count += 1;
    }
}

fn visit_aa_line(map: &mut HashMap<String, i32>, overlap_count: &mut i32, i: i32, j0: i32, j1: i32, i_is_x: bool) {
    
    let (j0, j1) = if j0 < j1 { (j0, j1) } else { (j1, j0) };
    for j in j0..=j1 {
        let (x, y) = if i_is_x { (i, j) } else { (j, i) };
        visit(map, overlap_count, x, y);
    }
}

fn visit_diag_line(map: &mut HashMap<String, i32>, overlap_count: &mut i32, x0: i32, y0: i32, x1: i32, y1: i32) {
    
    let (x0, y0, x1, y1) = if x0 < x1 { (x0, y0, x1, y1) } else { (x1, y1, x0, y0) };
    let ky = if y0 < y1 { 1 } else { -1 };
    if x1 - x0 != (y1 - y0) * ky {
        panic!("non diagonal {:?}", ((x0, y0), (x1, y1)));
    }
    for x in x0..=x1 {
        let y = y0 + (x - x0) * ky;
        visit(map, overlap_count, x, y);
    }
}

fn solve(lines: &[[[i32;2]; 2]], is_pt2: bool) -> i32 {
    
    let mut map: HashMap<String, i32> = HashMap::new();
    let mut overlap_count = 0;
    for line in lines {
        let x0 = line[0][0];
        let y0 = line[0][1];
        let x1 = line[1][0];
        let y1 = line[1][1];
        if x0 == x1 {
            visit_aa_line(&mut map, &mut overlap_count, x0, y0, y1, true);
        }
        else if y0 == y1 {
            visit_aa_line(&mut map, &mut overlap_count, y0, x0, x1, false);
        }
        else if is_pt2 {
            visit_diag_line(&mut map, &mut overlap_count, x0, y0, x1, y1);
        }
    }
    overlap_count
}

pub fn main() {
    
    let test_lines = vec![
        [[0,9], [5,9]],
        [[8,0], [0,8]],
        [[9,4], [3,4]],
        [[2,2], [2,1]],
        [[7,0], [7,4]],
        [[6,4], [2,0]],
        [[0,9], [2,9]],
        [[3,4], [1,4]],
        [[0,0], [8,8]],
        [[5,5], [8,2]],
    ];
    let day5_lines = read_lines("input/day5.txt");
    println!("test pt1 {:?}", solve(&test_lines, false));
    println!("day5 pt1 {:?}", solve(&day5_lines, false));
    println!("test pt2 {:?}", solve(&test_lines, true));
    println!("day5 pt2 {:?}", solve(&day5_lines, true));
}

fn read_lines<P>(filename: P) -> Vec<[[i32; 2]; 2]>
where P: AsRef<Path> {

    let file = File::open(filename).unwrap();
    let file_lines: Vec<String> = io::BufReader::new(file).lines().map(|v| v.unwrap()).collect();
    let mut lines = Vec::new();
    for line in file_lines.iter() {
        let line: [[i32; 2]; 2] = line.split(" -> ").map(|s| {
            s.split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .try_into().unwrap()
        })
        .collect::<Vec<[i32; 2]>>()
        .try_into().unwrap();
        lines.push(line);
    }
    lines
}