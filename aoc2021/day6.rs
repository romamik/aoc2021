use std::fs;

const CYCLE: i32 = 7;
const NEWBORN_CYCLE: i32 = 9;
const DAYS_PT1: i32 = 80;
const DAYS_PT2: i32 = 256;

fn naive_solve(fish: &[i32], days: i32) -> usize { 
    let mut all_fish: Vec<i32> = fish.to_vec();
    for _day in 1..=days {
        let mut new_fish = 0;
        for fish in all_fish.iter_mut() {
            if *fish == 0 {
                *fish = CYCLE - 1;
                new_fish += 1;
            }
            else {
                *fish -= 1;
            }
        }
        for _ in 0..new_fish {
            all_fish.push(NEWBORN_CYCLE - 1);
        }
    }
    all_fish.len()
}

fn still_naive_solve(fish: &[i32], days: i32) -> usize { 

    fn add_fish(all_fish: &mut Vec<(i32, usize)>, fish: i32, count: usize) {
        match all_fish.iter().position(|(f, _)| *f == fish) {
            Some(p) => all_fish[p].1 += count,
            None => all_fish.push((fish, count))
        }
    }

    let mut all_fish: Vec<(i32, usize)> = Vec::new();
    let mut new_fish: Vec<(i32, usize)> = Vec::new();

    for &fish in fish.iter() {
        add_fish(&mut all_fish, fish, 1);
    }

    for _day in 1..=days {
        let mut new_fish_count: usize = 0;
        for (fish, count) in all_fish.iter_mut() {
            if *fish == 0 {
                new_fish_count += *count
            }
            *fish = (CYCLE + *fish - 1) % CYCLE;
        }
        drain_filter(&mut new_fish, |(fish, count)| {
            *fish -= 1;
            if *fish > CYCLE {
                false
            }
            else {
                add_fish(&mut all_fish, *fish, *count);
                true
            }
        });
        if new_fish_count > 0 {
            new_fish.push((NEWBORN_CYCLE - 1, new_fish_count));
        }
    }
    all_fish.append(&mut new_fish);
    all_fish.iter().fold(0, |sum, (_, count)| sum + count)
}

fn main() {
    
    let test_fish = vec![3,4,3,1,2];
    let day6_fish = read_array("day6.txt");
    println!("naive test pt1 {}", naive_solve(&test_fish, DAYS_PT1));
    println!("naive day6 pt1 {}", naive_solve(&day6_fish, DAYS_PT1));
    println!("less naive test pt1 {}", still_naive_solve(&test_fish, DAYS_PT1));
    println!("less naive day6 pt1 {}", still_naive_solve(&day6_fish, DAYS_PT1));
    println!("less naive test pt2 {}", still_naive_solve(&test_fish, DAYS_PT2));
    println!("less naive day6 pt2 {}", still_naive_solve(&day6_fish, DAYS_PT2));
}

fn drain_filter<T, F>(vec: &mut Vec<T>, mut pred: F) 
where F: FnMut(&mut T) -> bool {
    let mut i = 0;
    while i < vec.len() {
        if pred(&mut vec[i]) {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

fn read_array(filename: &str) -> Vec<i32> {
    let s = fs::read_to_string(filename).unwrap();
    s.split(",")
    .map(|s| s.parse().unwrap())
    .collect::<Vec<i32>>()
}