use std::fs;

fn make_step(state: &mut [i8; 100]) -> usize {
    let mut num_flashes = 0;
    let mut flashes = Vec::new();
    for y in 0..10_i8 {
        for x in 0..10_i8 {
            let off = (y * 10 + x) as usize;
            state[off] += 1;
            if state[off] == 10 {
                flashes.push((x, y));
                num_flashes += 1;
            }
        }
    }
    while let Some(flash) = flashes.pop() {
        for y in flash.1 - 1..=flash.1 + 1 {
            for x in flash.0 - 1..=flash.0 + 1 {
                if x >= 0 && x < 10 && y >= 0 && y < 10 {
                    let off = (y * 10 + x) as usize;
                    state[off] += 1;
                    if state[off] == 10 {
                        flashes.push((x, y));
                        num_flashes += 1;
                    }
                }
            }
        }
    }
    for i in 0..100 {
        if state[i] > 9 {
            state[i] = 0;
        }
    }
    num_flashes
}

fn solve_pt1(input: &[i8; 100]) -> usize {
    let mut map = *input;
    let mut num_flashes = 0;
    for i in 0..100 {
        num_flashes += make_step(&mut map);
    }
    num_flashes
}

fn solve_pt2(input: &[i8; 100]) -> usize {
    let mut map = *input;
    let mut step = 0;
    loop {
        step += 1;
        let num_flashes = make_step(&mut map);
        if num_flashes == 100 {
            return step;
        }
    }
}

pub fn main() {
    let test_input = "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    ";
    let test_input = parse_input(test_input);
    let day11_input = parse_input(&fs::read_to_string("input/day11.txt").unwrap());

    println!("test pt1 {}", solve_pt1(&test_input));
    println!("day11 pt1 {}", solve_pt1(&day11_input));
    println!("test pt2 {}", solve_pt2(&test_input));
    println!("day11 pt2 {}", solve_pt2(&day11_input));
}

fn to_string(state: &[i8; 100]) -> String {
    let mut result = String::new();
    for y in 0..10 {
        let line: String = state[y * 10..y * 10 + 10]
            .iter()
            .cloned()
            .map(|c| if c < 10 { c + '0' as i8 } else { c - 10 + 'A' as i8 } as u8 as char)
            .collect();
        result.push_str(&line);
        result.push('\n');
    }
    result
}

fn parse_input(s: &str) -> [i8; 100] {
    <[i8; 100]>::try_from(
        s.bytes()
            .filter(|&c| c >= '0' as u8 && c <= '9' as u8)
            .map(|c| (c - '0' as u8) as i8)
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
