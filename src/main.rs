mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let run_all = true;
    if run_all {
        day1::main();
        day2::main().unwrap();
        day3::main();
        day4::main();
        day5::main();
        day6::main();
        day7::main();
    }
    day8::main();
}
