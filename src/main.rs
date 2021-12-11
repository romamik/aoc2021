mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
    let run_all = false;
    if run_all {
        day1::main();
        day2::main().unwrap();
        day3::main();
        day4::main();
        day5::main();
        day6::main();
        day7::main();
        day8::main();
        day9::main();
        day10::main();
    }
    day11::main();
}
