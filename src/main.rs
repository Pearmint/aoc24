use crate::day1::run_day1;
use crate::day2::run_day2;
use crate::day3::run_day3;


#[path = "01/01.rs"] mod day1;
#[path = "02/02.rs"] mod day2;
#[path = "03/03.rs"] mod day3;

fn main() {
    println!("Running Advent of Code 2024: A rusty journey!\n___");
    let file_path_d1:&str = "src/01/input";
    run_day1(file_path_d1);
    let file_path_d1:&str = "src/02/input";
    run_day2(file_path_d1);
    let file_path_d1:&str = "src/03/input";
    run_day3(file_path_d1);
}
