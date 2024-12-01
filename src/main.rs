use day1::{run_day1_1, run_day1_2};

#[path = "01/01.rs"] mod day1;

fn main() {
    println!("Running Advent of Code 2024: A rusty journey!");
    let file_path:&str = "src/01/input";
    run_day1_1(file_path);
    run_day1_2(file_path);
    println!("_________")
}
