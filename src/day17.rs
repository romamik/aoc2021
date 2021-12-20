#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rect {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Rect {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Rect {
        assert!(min_x > 0 && min_x < max_x && min_y < max_y);
        Rect {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

#[derive(PartialEq, Debug)]
enum SimulateResult {
    Hit { max_height: i32 },
    Undershoot,
    BelowTarget,
    AboveTarget,
    MayBeThroughTarget,
}

fn simulate(init_vel: &Point, target: &Rect) -> SimulateResult {
    let mut pos = Point::new(0, 0);
    let mut vel = init_vel.clone();
    let mut max_height = pos.y;
    let mut last_y_before_target = pos.y;
    loop {
        pos.x += vel.x;
        pos.y += vel.y;
        //println!("{:?}", pos);

        max_height = std::cmp::max(max_height, pos.y);

        if pos.x < target.min_x {
            if vel.x == 0 {
                return SimulateResult::Undershoot;
            }
            last_y_before_target = pos.y;
        } else if pos.x <= target.max_x {
            if pos.y < target.min_y {
                if vel.x == 0 {
                    return if last_y_before_target < target.min_y {
                        SimulateResult::BelowTarget
                    } else {
                        SimulateResult::MayBeThroughTarget
                    };
                }
            } else if pos.y <= target.max_y {
                return SimulateResult::Hit { max_height };
            }
        } else {
            // pos.x > target.max_x

            return if pos.y > target.max_y && last_y_before_target > target.max_y {
                SimulateResult::AboveTarget
            } else if pos.y < target.min_y && last_y_before_target < target.min_y {
                SimulateResult::BelowTarget
            } else {
                SimulateResult::MayBeThroughTarget
            };
        }

        if vel.x > 0 {
            vel.x -= 1;
        } else if vel.x < 0 {
            vel.x += 1;
        }

        vel.y -= 1;
    }
}

#[allow(dead_code)]
struct Trajectory {
    init_vel: Point,
    max_height: i32,
}

fn find_all_hit_trajectories(target: &Rect) -> Vec<Trajectory> {
    let min_vx = 1; // we should throw in direction of target
    let max_vx = target.max_x; // if we throw at greater speed we will be farther than target after first step

    let min_vy = target.min_y; // if throw at lower (greater in absolute value, but in negative direction) speed will be below target after first step
    let max_vy = -min_vy; // if we throw up probe will go up and than return and at starting position it will have same absolute velocity but in down direction

    let mut trajectories = Vec::new();
    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            if let SimulateResult::Hit { max_height } = simulate(&Point::new(vx, vy), &target) {
                trajectories.push(Trajectory {
                    init_vel: Point::new(vx, vy),
                    max_height,
                });
            }
        }
    }
    trajectories
}

fn find_max_height(target: &Rect) -> i32 {
    find_all_hit_trajectories(target).iter().max_by_key(|t| t.max_height).unwrap().max_height
}

fn count_trajectories(target: &Rect) -> usize {
    find_all_hit_trajectories(target).iter().count()
}

pub fn main() {
    test_simulate();
    test_find_max_height();
    test_count_trajectories();
    let day17_target = Rect::new(56, 76, -162, -134);
    println!("day 17 pt1 {}", find_max_height(&day17_target));
    println!("day 17 pt2 {}", count_trajectories(&day17_target));
}

fn test_count_trajectories() {
    let target = Rect::new(20, 30, -10, -5);
    assert_eq!(count_trajectories(&target), 112);
}

fn test_find_max_height() {
    let target = Rect::new(20, 30, -10, -5);
    assert_eq!(find_max_height(&target), 45);
}

fn test_simulate() {
    let target = Rect::new(20, 30, -10, -5);
    let tests = [
        (6, -472054, SimulateResult::BelowTarget),
        (7, 2, SimulateResult::Hit { max_height: 3 }),
        (6, 3, SimulateResult::Hit { max_height: 6 }),
        (9, 0, SimulateResult::Hit { max_height: 0 }),
        (17, -4, SimulateResult::MayBeThroughTarget),
        (17, -17, SimulateResult::BelowTarget),
        (1, 0, SimulateResult::Undershoot),
        (0, 0, SimulateResult::Undershoot),
        (17, 0, SimulateResult::AboveTarget),
    ];
    for test in tests.iter() {
        let pt = Point::new(test.0, test.1);
        let result = simulate(&pt, &target);
        assert_eq!(result, test.2, "{:?}", test);
    }
}
