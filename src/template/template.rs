use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DAY: &str = "Day 3";

pub fn run_day3(file_path: &str){
    println!("{}", DAY);
    run_day_1(file_path);
    run_day_2(file_path);
    println!("___");
}

pub fn run_day_1(file_path: &str){
    //println!("Calculating solution for {}_1", DAY);

    let val = 0;

    println!("{}_1 Solution: {}", DAY, val)
    // print val
}

pub fn run_day_2(file_path: &str){
    //println!("Calculating solution for {}_2", DAY);
    let val = 0;
    println!("{}_1 Solution: {}", DAY, val)
    // print val
}




// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}