use std::{char, usize};

fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");

    let mut result: i32 = 0;
    let mut matrix: Vec<Vec<char>> = vec![]; 
    let mut start_positions: Vec<(usize,usize)> = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut line_vec: Vec<char> = vec![]; 
        for (x, c) in line.chars().enumerate() {
            if c == 'X' {
                start_positions.push((x,y))
            }
            line_vec.push(c)
        }
        matrix.push(line_vec)
    }
    let directions = &vec![
        (-1,0), 
        (-1,-1), 
        (0,-1), 
        (1,-1), 
        (1,0), 
        (1,1), 
        (0,1), 
        (-1,1), 
    ];
    for pos in start_positions {
        for direction in directions {
            if is_xmas(pos, *direction, &matrix, 0) {
                result +=1
            }

        }
    }
    println!("{}", result);
}

fn is_xmas(
    u_pos: (usize, usize), 
    dir: (i32,i32), 
    matrix: &Vec<Vec<char>>,
    mut index: usize,
) -> bool {
    if index > 2 { return true };
    //println!("\n({},{}): {} (index {})", u_pos.0, u_pos.1, matrix[u_pos.1][u_pos.0], index);
    let pos = (u_pos.0 as i32, u_pos.1 as i32);
    if can_move(pos, dir, &matrix) 
    {
        let m_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if matrix[m_pos.1 as usize][m_pos.0 as usize] == "MAS".chars().nth(index).unwrap() {
            index += 1;
            let new_u_pos = (m_pos.0 as usize,m_pos.1 as usize);
            return is_xmas(new_u_pos, dir, matrix, index)
        } else { return false }
    }
    false
}

fn can_move(
    pos: (i32, i32), 
    direction: (i32,i32), 
    matrix: &Vec<Vec<char>>
) -> bool {
    pos.0 + direction.0 >= 0 && pos.0 + direction.0 < matrix[0].len() as i32 &&
    pos.1 + direction.1 >= 0 && pos.1 + direction.1 < matrix.len() as i32
}
