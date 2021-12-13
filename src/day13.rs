use std::collections::HashSet;
use std::cmp;

type Axis = usize;

const X_AXIS: Axis = 0;
const Y_AXIS: Axis = 1;

#[derive(Debug)]
struct Input {
    name: String,
    dots: HashSet::<[usize; 2]>,
    folds: Vec::<(Axis, usize)> 
}

fn fold(dots: &HashSet::<[usize; 2]>, fold: &(Axis, usize)) -> HashSet::<[usize; 2]> {
    let mut result = HashSet::new();
    for dot in dots.iter() {
        let mut new_dot = dot.clone();
        if new_dot[fold.0] > fold.1 {
            new_dot[fold.0] = 2*fold.1 - new_dot[fold.0];
        }
        result.insert(new_dot);
    }
    result
}

fn solve_pt1(input: &Input) -> usize {

    let new_dots = fold(&input.dots, &input.folds[0]);
    new_dots.iter().count()
}

fn solve_pt2(input: &Input) -> String {

    let mut dots = input.dots.clone();
    for f in input.folds.iter() {
        dots = fold(&dots, &f);
    }

    let max = dots.iter().fold([1, 1], |acc, &dot| [cmp::max(acc[0], dot[0]), cmp::max(acc[1], dot[1])]);
    let max = [max[0] + 1, max[1] + 1];
    let mut vec = vec![false; max[X_AXIS] * max[Y_AXIS]];
    dots.iter().for_each(|dot| vec[dot[X_AXIS] + dot[Y_AXIS] * max[X_AXIS]] = true);
    let mut result = String::new();
    for y in 0..max[Y_AXIS] {
        for x in 0..max[X_AXIS] {
            result.push(if vec[x + y * max[X_AXIS]] { '#' } else { ' ' });
        }
        result.push('\n');
    }

    result
}

pub fn main() {
    for input in read_input("input/day13.txt") {
        println!("{} pt1 {}", input.name, solve_pt1(&input));
        println!("{} pt2:\n{}", input.name, solve_pt2(&input));
    }
}

fn read_input(filename: &str) -> Vec::<Input> {
    let s =std::fs::read_to_string(filename).unwrap();
    let mut lines = s.split("\n").map(|s| s.trim()).collect::<Vec<_>>();
    lines.reverse();
    let mut result = Vec::new();
    while let Some(name) = lines.pop() {
        if name.len() == 0 {
            continue;
        }
        let mut dots = HashSet::new();
        while let Some(line) = lines.pop() {
            if line.len() == 0 {
                break;
            }
            let dot = <[usize; 2]>::try_from(line.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>()).unwrap();
            dots.insert(dot);
        }
        let mut folds = Vec::new();
        while let Some(line) = lines.pop() {
            if line.len() == 0 {
                break;
            }
            let t = <[&str; 2]>::try_from(line.split("=").collect::<Vec<_>>()).unwrap();
            let coordinate = t[1].parse::<usize>().unwrap();
            let axis = match t[0] {
                "fold along x" => X_AXIS,
                "fold along y" => Y_AXIS,
                v => panic!("bad axis {}", v),
            };
            folds.push((axis, coordinate));
        }
        result.push(Input {name: name.to_string(), dots, folds});
    }
    result
}
