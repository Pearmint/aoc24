use regex::Regex;
use std::fs::read_to_string;

const DAY: &str = "Day 3";

pub fn run_day3(file_path: &str){
    println!("{}", DAY);
    run_day_1(file_path);
    run_day_2(file_path);
    println!("___");
}

pub fn run_day_1(file_path: &str){
    //println!("Calculating solution for {}_1", DAY);
    let (left, right) = parse_input_1(file_path);
    let val = calc_mults(left, right);
    println!("{}_1 Solution: {}", DAY, val)
    // print val
}

pub fn run_day_2(file_path: &str){
    //println!("Calculating solution for {}_2", DAY);

    let captures = parse_input_2(file_path);
    
    let mut left = vec![];
    let mut right = vec![];

    for cap in captures.iter() {
        let (mut l,mut r) = extract_nums(cap);
        left.append(&mut l);
        right.append(&mut r);
    }

    let val = calc_mults(left, right);
    println!("{}_2 Solution: {}", DAY, val)
    // print val
}

fn parse_input_1(file_path: &str) ->  (Vec<i32>,Vec<i32>){
    //TODO error handling
    let mut left = Vec::new();
    let mut right = Vec::new();
    
    if let Ok(content) = read_to_string(file_path) {
        (left,right) = extract_nums(&content);
    }
    return (left, right);
}

fn extract_nums(content: &str) ->(Vec<i32>,Vec<i32>){

    let mut left = Vec::new();
    let mut right = Vec::new();
    let re = Regex::new(r"mul\((\d+)\,(\d+)\)").unwrap();
    for cap in re.captures_iter(&content){

        let l = cap[1].parse::<i32>().unwrap();
        let r = cap[2].parse::<i32>().unwrap();
        //println!("{},{}",l,r);

        left.push(l);
        right.push(r);
    }
    return (left, right);
}

fn calc_mults(mut left: Vec<i32>, mut right: Vec<i32>) -> i32{
    let mut val = 0;
    if left.len() != right.len() {
        panic!("Both sides are not equally long!");
    }

    while left.len() > 0 {
        val += left.pop().unwrap() * right.pop().unwrap();
    }
    return val;
}

fn parse_input_2(file_path: &str) ->  Vec<String>{
    //TODO error handling
    let mut captures: Vec<String> = Vec::new();
    let re = Regex::new(r"(.*)?don't\(\)|do\(\)(.*)").unwrap();

    if let Ok(content) = read_to_string(file_path) {
        for cap in re.captures_iter(&content){
            if cap.get(1).is_some() {
                //println!("{}",c.unwrap().as_str().to_string());
                captures.push(cap[1].to_string());
            }
            if cap.get(2).is_some() {
                //println!("{}",c.unwrap().as_str().to_string());
                captures.push(cap[2].to_string());
            }
        }
    }
    return captures;
}
