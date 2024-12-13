use std::process::Command;

type Matrix = Vec<Vec<char>>;
#[derive(Clone, Debug)]
struct Plot {
    character: char,
    area: u64,
    perimeter: u64
}

fn main() {
    Command::new("clear").status().unwrap();
    //let input = include_str!("./test3.txt");
    let input = include_str!("./input1.txt");
    let mut matrix: Matrix = vec![];
    let mut visited_positions: Vec<(usize, usize)> = vec![];
    let mut plots: Vec<Plot> = vec![];

    generate_matrix(&input, &mut matrix);
    //for line in &matrix { println!("{:?}", line);}
    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !visited_positions.contains(&(x,y)) {
                let mut new_plot = Plot {
                    character: *c,
                    area: 0,
                    perimeter: 0
                };
                explore_plot(
                    &matrix, 
                    &mut visited_positions, 
                    *c, 
                    (x as i64, y as i64), 
                    &mut new_plot
                );
                plots.push(new_plot)
            }
        }
    }
    //for plot in &plots { println!("{:?}", plot) }
    let result = calc_result(plots);
    println!("Result {}", result)
}

fn calc_result(plots: Vec<Plot>) -> u64 {
    let mut result: u64 = 0;
    for plot in plots {
        result += plot.area * plot.perimeter
    }
    result
}

fn explore_plot(
    matrix: &Matrix,
    visited_positions: &mut Vec<(usize, usize)>,
    c: char,
    coord: (i64, i64),
    new_plot: &mut Plot,
) {
    println!("Checking {:?} for {}", coord, c);
    let out_of_bounds = coord.0 < 0 || coord.1 < 0 || coord.0 > matrix[0].len() as i64 || coord.1 > matrix.len() as i64;
    if out_of_bounds { return }

    let cur = matrix[coord.1 as usize][coord.0 as usize];
    if c != cur { return }

    visited_positions.push((coord.0 as usize, coord.1 as usize));
    new_plot.area += 1;
    new_plot.perimeter += perimeter(matrix, c, coord);

    let dirs: Vec<(i64,i64)> = vec![(-1,0), (0,-1), (1,0), (0,1)];
    for dir in dirs {
        let out_of_bounds = coord.0 + dir.0 < 0 || coord.1 + dir.1 < 0 || 
            coord.0 + dir.0 >= matrix[0].len() as i64 || coord.1 + dir.1 >= matrix.len() as i64;
        if !out_of_bounds && 
            matrix[(coord.1 + dir.1) as usize][(coord.0 + dir.0) as usize] == c &&
            !visited_positions.contains(&((coord.0 + dir.0) as usize, (coord.1 + dir.1) as usize))
        {
            explore_plot(
                matrix, 
                visited_positions, 
                c, 
                (coord.0 + dir.0, coord.1 + dir.1), 
                new_plot
            );
        }
    }
}

fn perimeter(matrix: &Matrix, c: char, coord: (i64, i64)) -> u64 {
    let mut perimeter: u64 = 0;
    let dirs: Vec<(i64,i64)> = vec![(-1,0), (0,-1), (1,0), (0,1)];
    for dir in dirs {
        let out_of_bounds = coord.0 + dir.0 < 0 || coord.1 + dir.1 < 0 || 
            coord.0 + dir.0 >= matrix[0].len() as i64 || coord.1 + dir.1 >= matrix.len() as i64;
        if out_of_bounds || matrix[(coord.1 + dir.1) as usize][(coord.0 + dir.0) as usize] != c {
            perimeter += 1
        }
    }
    perimeter
}

fn generate_matrix(input: &str, matrix: &mut Matrix) {
    for line in input.lines() {
        let mut line_vec: Vec<char> = vec![];
        for c in line.chars() {
            line_vec.push(c);
        }
        matrix.push(line_vec);
    }
}

