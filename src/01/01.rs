use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn run_day1(file_path: &str){
    println!("Day 1");
    run_day1_1(file_path);
    run_day1_2(file_path);
    println!("___");
}

pub fn run_day1_1(file_path: &str){
    //println!("Calculating solution for Day 1_1");

    let (mut left, mut right) = parse_input(file_path);

    if left.len() != right.len() {
        panic!("Both sides are not equally long!");
    }
    // sort arrays ascending
    left.sort();
    right.sort();

    let mut val = 0;

    while left.len() > 0 {
        val += (left.pop().unwrap() - right.pop().unwrap()).abs();
    }


    println!("01_1 Solution: {}", val)
    // print val
}

pub fn run_day1_2(file_path: &str){
    //println!("Calculating solution for Day 1_2");

    let (mut left, mut right) = parse_input(file_path);

    if left.len() != right.len() {
        panic!("Both sides are not equally long!");
    }
    // sort arrays ascending
    left.sort();
    right.sort();

    let mut val = 0;

    while left.len() > 0 {
        let n = left.pop().unwrap();
        val += n * count_of_elem_in_vec(n, right.clone());
    }


    println!("01_2 Solution: {}", val)
    // print val
}

fn count_of_elem_in_vec(n: i32, vec: Vec<i32>) -> i32 {
    return vec.iter().filter(|&x| *x == n).count().try_into().unwrap();
}


fn parse_input(file_path: &str) ->  (Vec<i32>,Vec<i32>){
    //error handling
    let mut left = Vec::new();
    let mut right = Vec::new();
    let re = Regex::new(r"^(\d*)\s{3}(\d*)$").unwrap();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {

            // parse input 
            let cap = re.captures(&line).unwrap();
            let l: i32 = cap[1].parse::<i32>().unwrap();
            let r: i32 = cap[2].parse::<i32>().unwrap();

            //println!("{} and {} read.", l, r);
            // add to array
            left.push(l);
            right.push(r);
        }
    }
    return (left, right);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}