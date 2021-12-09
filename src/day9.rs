use std::fs;

type HeightMap = Vec<Vec<u8>>;

fn get_height_at(map: &HeightMap, x: i32, y: i32) -> Option<u8> {
    let line = if y >= 0 && y < map.len() as i32 {
        Some(&map[y as usize])
    } else {
        None
    };

    match line {
        Some(line) => {
            if x >= 0 && x < line.len() as i32 {
                Some(line[x as usize])
            } else {
                None
            }
        }
        None => None,
    }
}

fn find_lower_points(map: &HeightMap) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for y in 0..map.len() as i32 {
        let line = &map[y as usize];
        for x in 0..line.len() as i32 {
            let height = get_height_at(map, x, y).unwrap();
            let neighbors_heights = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(dx, dy)| get_height_at(map, x as i32 + dx, y as i32 + dy))
                .collect::<Vec<u8>>();
            let total_neighbors = neighbors_heights.len();
            let higher_neighbors = neighbors_heights
                .iter()
                .filter(|neighbor_height| **neighbor_height > height)
                .count();
            if higher_neighbors == total_neighbors {
                result.push((x, y));
            }
        }
    }
    result
}

fn solve_pt1(map: &HeightMap) -> i32 {
    let mut risk_sum = 0;
    for (x, y) in find_lower_points(map).iter() {
        let height = get_height_at(map, *x, *y).unwrap();
        risk_sum += 1 + height as i32;
    }
    risk_sum
}

fn solve_pt2(map: &HeightMap) -> usize {
    let mut basin_sizes = vec![];
    for (x, y) in find_lower_points(map).iter() {
        let mut basin = std::collections::HashSet::<(i32, i32)>::new();
        let mut queue = vec![(*x, *y)];
        while queue.len() > 0 {
            let (x, y) = queue.pop().unwrap();
            if basin.get(&(x, y)) == None {
                let h = get_height_at(map, x, y);
                if let Some(h) = h {
                    if h != 9 {
                        basin.insert((x, y));
                        queue.push((x + 1, y));
                        queue.push((x - 1, y));
                        queue.push((x, y + 1));
                        queue.push((x, y - 1));
                    }
                }
            }
        }
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0..3].iter().fold(1, |a, b| a * *b)
}

pub fn main() {
    let test_input = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";
    let test_input = parse_input(test_input);
    let day9_input = parse_input(&fs::read_to_string("input/day9.txt").unwrap());
    println!("test pt1 {}", solve_pt1(&test_input));
    println!("day9 pt1 {}", solve_pt1(&day9_input));
    println!("test pt2 {}", solve_pt2(&test_input));
    println!("day9 pt2 {}", solve_pt2(&day9_input));
}

fn parse_input(s: &str) -> HeightMap {
    s.split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|c| c - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
