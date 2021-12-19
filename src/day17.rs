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
                    }
                    else {
                        SimulateResult::MayBeThroughTarget
                    }
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

fn find_max_height(target: &Rect) -> i32 {

    let mut max_height = 0;
    for vx in 1..=target.max_x {

        let mut vy = 0;
        loop {
            match simulate(&Point::new(vx, vy), &target) {
                SimulateResult::Hit { max_height: mh } => {
                    max_height = std::cmp::max(max_height, mh);
                },
                SimulateResult::BelowTarget | SimulateResult::Undershoot => {
                    break;
                }
                _r => {
                    //println!("stage1 {} {} {:?}", vx, vy, _r);
                }
            }
            vy -= 1;
            
            // just cheat
            if vy < -10000 { break; }
        }
        vy = 1;
        loop {
            match simulate(&Point::new(vx, vy), &target) {
                SimulateResult::Hit { max_height: mh } => {
                    max_height = std::cmp::max(max_height, mh);
                },
                SimulateResult::AboveTarget | SimulateResult::Undershoot=> {
                    break;
                }
                _r => {
                    //println!("stage2 {} {} {:?}", vx, vy, _r);
                }
            }
            vy += 1;
            
            // just cheat
            if vy > 10000 { break; }
        }
    }
    max_height
}

pub fn main() {
    test_simulate();
    test_find_max_height();
    println!("day 17 pt1 {}", find_max_height(&Rect::new(56, 76, -162, -134)));
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

