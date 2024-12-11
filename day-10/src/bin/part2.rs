
use std::process::Command;

type Matrix = Vec<Vec<i8>>;

fn main() {
    Command::new("clear").status().unwrap();
    println!("\n\n");
    println!("Start");
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let input = input.trim();

    let mut final_result: i64 = 0;
    let mut matrix: Matrix = vec![];

    generate_matrix(input, &mut matrix);

    print_matrix(&matrix);

    for (y,line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let mut path: Vec<i8> = vec![];
            let mut result: i64 = 0;
            num_of_paths(
                (x as i64, y as i64), 
                &matrix, 
                -1, 
                &mut path, 
                &mut result
            );
            if result > 0 {
                println!("Point {:},{} score {}", x,y, result)
            }
            final_result += result;
        }
    }

    println!("\n\nResult {}: ", final_result);
}

fn print_matrix(matrix: &Matrix) {
    matrix
    .iter()
    .for_each(|line| {
        println!("{:?}", line)
    });
}

fn num_of_paths (
    point: (i64,i64), 
    matrix: &Matrix,
    mut previous: i8,
    path: &mut Vec<i8>,
    result: &mut i64
) {
    let max_x = matrix[0].len() as i64 - 1;
    let max_y = matrix.len() as i64 - 1;
    if  point.0 < 0 || point.0 > max_x || 
        point.1 < 0 || point.1 > max_y {
        return
    }
    let current = matrix[point.1 as usize][point.0 as usize].clone();
    if current == previous + 1 {
        path.push(current);
    } else {
        return
    }
    if path.len() == 10 {
        *result += 1;
        return
    } else {
        previous += 1;
        num_of_paths((point.0 -1 , point.1), matrix, previous, &mut path.clone(), result);
        num_of_paths((point.0 + 1, point.1), matrix, previous, &mut path.clone(), result);
        num_of_paths((point.0, point.1 - 1), matrix, previous, &mut path.clone(), result);
        num_of_paths((point.0, point.1 + 1), matrix, previous, &mut path.clone(), result);
    }
}

fn generate_matrix(input: &str, matrix: &mut Matrix) {
    input
        .lines()
        .for_each(|line| {
            let mut line_vec: Vec<i8> = vec![];
            line
                .chars()
                .for_each(|c| {
                    let n = c.to_string().parse::<i8>().unwrap();
                    line_vec.push(n);
                });
            matrix.push(line_vec);
        });
}