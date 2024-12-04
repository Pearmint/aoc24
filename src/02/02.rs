use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DAY: &str = "Day 2";

pub fn run_day2(file_path: &str){
    println!("{}", DAY);
    run_day2_1(file_path);
    run_day2_2(file_path);
    println!("___");
}

pub fn run_day2_1(file_path: &str){
    //println!("Calculating solution for {}_1", DAY);

    let val = count_save_reports(file_path);

    println!("{}_1 Solution: {}", DAY, val)
    // print val
}

pub fn run_day2_2(file_path: &str){
    //println!("Calculating solution for {}_2", DAY);
    let val = count_save_reports_2(file_path);
    println!("{}_1 Solution: {}", DAY, val)
    // print val
}

fn count_save_reports(file_path: &str) ->  i32{
    //error handling
    let mut safe_count: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<i32>>();
            let res = check_safe(nums);
            //println!("{}: {}", res, line);
            if res {
                safe_count += 1;
            }
        }
    }
    return safe_count;
}

fn count_save_reports_2(file_path: &str) ->  i32{
    //error handling
    let mut safe_count: i32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<i32>>();
            let res = check_safe_2(nums);
            //println!("{}: {}", res, line);
            if res {
                safe_count += 1;
            }
        }
    }
    return safe_count;
}


fn check_safe(mut numbers: Vec<i32>) -> bool{
    let mut first;
    let mut second;
    let direction: bool;

    first = numbers.pop().unwrap();
    second = numbers.pop().unwrap();


    direction = first < second;

    let distance = (first - second).abs();

    if (distance < 1) || (distance > 3) {
        return false;
    }

    while numbers.len() > 0 {
        first = second;
        second = numbers.pop().unwrap();
        
        if (first < second) != direction {
            return false;
        }

        let distance = (first - second).abs();

        if (distance < 1) || (distance > 3) {
            return false;
        }
    }   
    return true;
}

fn check_safe_2(mut numbers: Vec<i32>) -> bool{
    let mut first;
    let mut second;
    let  third;
    let increasing: bool;
    let mut bad_level_count: i32 = 0;

    first = numbers.pop().unwrap();
    second = numbers.pop().unwrap();
    third = numbers.pop().unwrap();

    if !is_safe(first, second, first < second){
        if !is_safe(first, third, first < third){
            return false;
        } else {
            increasing = first < third;
            bad_level_count += 1;
            second = third;
        }
    } else {
        increasing = first < second;
    }

    while numbers.len() > 0 {
        first = second;
        second = numbers.pop().unwrap();

        if !is_safe(first, second, increasing){
            if bad_level_count > 0 {
                return false;
            }
            bad_level_count += 1;
            if !is_safe(first, third,increasing){
                return false;
            }
        }
    }
    
    return true;
}

fn is_safe(a: i32, b:i32, increasing: bool) -> bool{ 
    let distance = (a - b).abs();

    if (distance < 1) || (distance > 3) || (a < b) != increasing{
        return false;
    }
    return true;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}