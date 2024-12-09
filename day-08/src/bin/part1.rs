use std::{collections::HashMap, i64};

type Antinode = (i32,i32);
type Antinodes = Vec<Antinode>;
type InputMap = HashMap<char, Antinodes>;

struct Boundaries {
    x: (i32, i32),
    y: (i32, i32)
}

fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let mut antinodes: Antinodes = vec![];
    let mut input_map: InputMap = HashMap::new();
    let mut boundaries = Boundaries {
        x: (0, 0),
        y: (0, 0)
    };

    generate_input_map(
        input, 
        &mut input_map, 
        &mut boundaries
    );
    println!("{:?}", input_map);
    
    for (_frequency, antennas) in input_map.iter() {
        if antennas.len() >= 2 {
            update_antinodes(
                antennas,
                &mut antinodes,
                &boundaries
            );
        }
    }

    println!("Final result: {}", antinodes.len());
    println!("Antinodes: \n\n {:?}", antinodes);
}

fn update_antinodes (
    antennas: &Vec<(i32,i32)>,
    antinodes: &mut Antinodes,
    boundaries: &Boundaries
) {
    let mut i = 0;
    let mut j = 1;

    while i < antennas.len() - 1 {
        let antenna1 = &antennas[i];
        while j < antennas.len() {
            let antenna2 = &antennas[j];
            check_antinodes(
                antenna1, 
                antenna2,
                antinodes,
                boundaries
            );
            j += 1;
        }
        i += 1;
        j = i + 1
    }
}

fn check_antinodes(
    antenna1: &(i32,i32), 
    antenna2: &(i32,i32),
    antinodes: &mut Antinodes,
    boundaries: &Boundaries
) {
    let distance_x = antenna1.0 - antenna2.0;
    let distance_y = antenna1.1 - antenna2.1;

    let new_antinode1_x = antenna1.0 + distance_x;
    let new_antinode1_y = antenna1.1 + distance_y;
    let new_antinode = (new_antinode1_x, new_antinode1_y);

    if check_boundaries(&new_antinode, boundaries) &&
        !antinodes.contains(&new_antinode)
     { antinodes.push(new_antinode) }

    let new_antinode2_x = antenna2.0 - distance_x;
    let new_antinode2_y = antenna2.1 - distance_y;
    let new_antinode = (new_antinode2_x, new_antinode2_y);

    if check_boundaries(&new_antinode, boundaries) &&
        !antinodes.contains(&new_antinode)
     { antinodes.push(new_antinode) }
}

fn check_boundaries(
    antinode: &Antinode,
    boundaries: &Boundaries
) -> bool {
    antinode.0 >= 0 &&
    antinode.0 <= boundaries.x.1 &&
    antinode.1 >= 0 &&
    antinode.1 <= boundaries.y.1
}

fn generate_input_map(
    input: &str,
    input_map: &mut InputMap,
    boundaries: &mut Boundaries
) {
    let mut boundary_y = 0;
    for (y, line) in input.lines().enumerate() {
        let mut boundary_x = 0;
        for (x, c) in line.chars().enumerate() {
            boundary_x = x;
            if c != '.' {
                match input_map.get_mut(&c) {
                    Some(v) => { v.push((x as i32,y as i32)) },
                    None => { input_map.insert(c, vec![(x as i32, y as i32)]); }
                }
            }
        }
        if y == 0 {
            boundaries.x = (0, boundary_x as i32);
        }
        boundary_y = y;
    }
    boundaries.y = (0, boundary_y as i32);
}

#[allow(dead_code)]
fn generate_input_map2(
    input: &str,
    input_map: &mut InputMap 
) {
    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x,c)| {
                    if c != '.' {
                        match input_map.get_mut(&c) {
                            Some(v) => { v.push((x as i32,y as i32)) },
                            None => { input_map.insert(c, vec![(x as i32, y as i32)]); }
                        }
                    }
                })
        })
}
