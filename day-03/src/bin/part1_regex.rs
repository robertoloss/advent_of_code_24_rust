//Taken from https://github.com/rust-dd/aoc-2024/blob/main/src/aoc_2024_03.result

use regex::Regex;

fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let matching_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = matching_regex.captures_iter(&input);
    
    let mut sum = 0;
    for cap in caps {
        let first_value = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second_value = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum = sum + (first_value * second_value);
    }

    println!("The result is {}", sum)
}

