use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
struct CubeCommand {
    on: bool,
    x: [isize; 2],
    y: [isize; 2],
    z: [isize; 2],
}

// world consists on cubes, each cube can be on or off
// cubes are formed by axis aligned planes
struct World {
    x: Vec<isize>, // sorted list of yz planes coordinates
    y: Vec<isize>,
    z: Vec<isize>,
    state: Vec<Vec<Vec<bool>>>, // state[iz][iy][ix] state of cube {x[ix], y[iy], z[iz]}-{x[ix+1], y[iy+1], z[iz+1]}
}

fn prepare_world(cmds: &[CubeCommand]) -> World {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();

    for cmd in cmds.iter() {
        for (world_coords, cmd_borders) in [(&mut x, &cmd.x), (&mut y, &cmd.y), (&mut z, &cmd.z)] {
            world_coords.push(cmd_borders[0]);
            world_coords.push(cmd_borders[1] + 1);
        }
    }

    for vec in [&mut x, &mut y, &mut z].into_iter() {
        vec.sort();
        vec.dedup();
    }

    let state = vec![vec![vec![false; x.len()]; y.len()]; z.len()];
    World { x, y, z, state }
}

fn apply_command(world: &mut World, cmd: &CubeCommand) {
    fn find_indices(world_coords: &[isize], cmd_borders: &[isize; 2]) -> (usize, usize) {
        (
            world_coords.binary_search(&cmd_borders[0]).unwrap(),
            world_coords.binary_search(&(cmd_borders[1] + 1)).unwrap(),
        )
    }

    let ix = find_indices(&world.x, &cmd.x);
    let iy = find_indices(&world.y, &cmd.y);
    let iz = find_indices(&world.z, &cmd.z);

    for x in ix.0..ix.1 {
        for y in iy.0..iy.1 {
            for z in iz.0..iz.1 {
                world.state[z][y][x] = cmd.on;
            }
        }
    }
}

fn apply_commands(world: &mut World, cmds: &[CubeCommand]) {
    for cmd in cmds.iter() {
        apply_command(world, cmd);
    }
}

fn make_world(cmds: &[CubeCommand]) -> World {
    let mut world = prepare_world(cmds);
    apply_commands(&mut world, cmds);
    world
}

fn count_on(world: &World) -> isize {
    let mut total_vol = 0;
    for iz in 0..world.z.len() - 1 {
        let z = [world.z[iz], world.z[iz + 1]];
        let dz = z[1] - z[0];
        for iy in 0..world.y.len() - 1 {
            let y = [world.y[iy], world.y[iy + 1]];
            let dy = y[1] - y[0];
            for ix in 0..world.x.len() - 1 {
                let x = [world.x[ix], world.x[ix + 1]];
                let dx = x[1] - x[0];
                let on = world.state[iz][iy][ix];
                if on {
                    let vol = dx * dy * dz;
                    total_vol += vol;
                }
            }
        }
    }
    total_vol
}

fn filter_commands(cmds: &[CubeCommand], size: isize) -> Vec<CubeCommand> {
    cmds.iter()
        .filter(|cmd| {
            ![cmd.x, cmd.y, cmd.z]
                .iter()
                .flatten()
                .any(|&c| c < -size || c > size)
        })
        .cloned()
        .collect()
}

fn solve_pt1(input: &[CubeCommand]) -> isize {
    let world = make_world(&filter_commands(input, 50));
    count_on(&world)
}

fn solve_pt2(input: &[CubeCommand]) -> isize {
    let world = make_world(input);
    count_on(&world)
}

pub fn main() {
    let input = parse_input(&std::fs::read_to_string("input/day22.txt").unwrap());
    assert_eq!(590784, solve_pt1(input.get("test").unwrap()));
    assert_eq!(474140, solve_pt1(input.get("test2").unwrap()));
    assert_eq!(2758514936282235, solve_pt2(input.get("test2").unwrap()));
    println!("tests ok");
    println!("day 22 pt1 {}", solve_pt1(input.get("day22").unwrap()));
    println!("day 22 pt2 {}", solve_pt2(input.get("day22").unwrap()));
}

fn parse_input(s: &str) -> HashMap<String, Vec<CubeCommand>> {
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
    let mut result = HashMap::new();
    s.split("\n\n").for_each(|s| {
        let lines: Vec<_> = s.split("\n").collect();
        let name = lines[0].to_string();
        let mut commands = Vec::new();
        for line in &lines[1..] {
            let cap = re.captures(line).unwrap();
            let r: Vec<_> = (2..=7)
                .map(|i| cap.get(i).unwrap().as_str().parse::<isize>().unwrap())
                .collect();
            let cmd = CubeCommand {
                on: cap.get(1).unwrap().as_str() == "on",
                x: [r[0], r[1]],
                y: [r[2], r[3]],
                z: [r[4], r[5]],
            };
            commands.push(cmd);
        }
        result.insert(name, commands);
    });
    result
}
