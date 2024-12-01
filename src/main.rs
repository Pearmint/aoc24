use day1::{run_day1};

#[path = "01/01.rs"] mod day1;

fn main() {
    println!("Running Advent of Code 2024: A rusty journey!\n___");
    let file_path_d1:&str = "src/01/input";
    run_day1(file_path_d1);
}
