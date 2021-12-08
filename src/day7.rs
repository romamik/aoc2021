use std::fs;

fn cost_pt1(from: i32, to: i32) -> i32 {
    (from - to).abs()
}

fn cost_pt2(from: i32, to: i32) -> i32 {
    let delta = (from - to).abs();
    (1 + delta) * delta / 2
}

fn solve<F>(pos: &[i32], cost_pred: F) -> i32 
where F: Fn(i32, i32) -> i32 {
    let &min = pos.iter().min().unwrap();
    let &max = pos.iter().max().unwrap();
    let mut min_cost = None;
    for align_pos in min..=max {
        let cost = pos.iter().fold(0, |cost, &p| cost + cost_pred(align_pos, p));
        if min_cost.map(|min_cost| cost < min_cost).unwrap_or(true) {
            min_cost = Some(cost)
        }
    }
    min_cost.unwrap()
}

pub fn main() {
    let test_input = vec![16,1,2,0,4,2,7,1,2,14];
    let day7_input = read_array("input/day7.txt");
    println!("test pt1 {}", solve(&test_input, cost_pt1));
    println!("day7 pt1 {}", solve(&day7_input, cost_pt1));
    println!("test pt2 {}", solve(&test_input, cost_pt2));
    println!("day7 pt2 {}", solve(&day7_input, cost_pt2));
}

fn read_array(filename: &str) -> Vec<i32> {
    let s = fs::read_to_string(filename).unwrap();
    s.split(",")
    .map(|s| s.parse().unwrap())
    .collect::<Vec<i32>>()
}