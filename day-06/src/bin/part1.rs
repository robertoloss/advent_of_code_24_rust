use std::{collections::HashMap, usize};

type Pair = (i32, i32);

fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let result: &mut i32 = &mut 1;
    let mut storage: HashMap<Pair,char> = HashMap::new();
    let mut start: Pair = (0,0);
    let directions: Vec<Pair> = vec![
        (-1,0), 
        (0,-1), 
        (1,0), 
        (0,1), 
    ];
    let mut visited: HashMap<Pair,bool> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            storage.insert((x as i32,y as i32), c);
            if c == '^' {
                start = (x as i32, y as i32);
            }
        }
    }
    visited.insert(start, true);
    movement(start, 1, &directions, storage, result, &mut visited);
    println!("Result: {}", result);
}


fn movement(
    start: Pair,
    direction_index: usize,
    directions: &Vec<Pair>,
    storage: HashMap<Pair,char>,
    result: &mut i32,
    visited: &mut HashMap<Pair,bool>
) {
    let new_start = (
        start.0 + directions[direction_index].0,
        start.1 + directions[direction_index].1
    );
    match storage.get(&new_start) {
        None => { return },
        Some(c) => {
            if *c != '#' {
                if visited.get(&new_start).is_none() {
                    *result += 1;
                    visited.insert(new_start, true);
                }
                movement(
                    (start.0 + directions[direction_index].0,
                        start.1 + directions[direction_index].1),
                    direction_index, 
                    directions,
                    storage, 
                    result,
                    visited
                )
            } else {
                movement(
                    start, 
                    if direction_index == directions.len() -1 { 0 } 
                    else { direction_index + 1 }, 
                    directions, 
                    storage, 
                    result,
                    visited
                )
            }        
        }
    } 
}
