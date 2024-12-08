enum Operation {
    Add,
    Mult
}

fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let mut result: i64 = 0;
    let mut equations: Vec<(i64,Vec<i64>)> = vec![];

    generate_equations(input, &mut equations);
    for equation in &equations {
        println!("{:?}", equation);
    }

    for equation in equations {
        let mut status = false;
        print!("\n\n");
        result += can_be_made_true(
            equation.0, 
            equation.1[0], 
            &equation.1, 
            0, 
            Operation::Add,
            &mut status
        );
        println!("\nFinal Result (tmp): {}", result);
    }

    println!("\n\n======\nFinal Result {}", result);
}

fn can_be_made_true(
    result: i64,
    mut tmp_result: i64,
    test_values: &Vec<i64>, 
    index: usize,
    operation: Operation,
    status: &mut bool
) -> i64 {
    if *status { return result }
    print!("\nWe are here: {} - {} ", result, test_values[index]);
    if index > 0 {
        tmp_result = if let Operation::Add = operation {
            tmp_result + test_values[index]
        } else {
            tmp_result * test_values[index]
        }
    }
    if index < test_values.len() - 1 {
        can_be_made_true(
            result, 
            tmp_result, 
            test_values, 
            index + 1, 
            Operation::Add,
            status
        );
        can_be_made_true(
            result, 
            tmp_result, 
            test_values, 
            index + 1, 
            Operation::Mult,
            status
        )
    } else {
        print!("result = {} {}", tmp_result, tmp_result==result);
        if tmp_result == result {
            *status = true;
            return result
        } else {
            return 0
        }
    } 
}

fn generate_equations(input: &str, equations: &mut Vec<(i64,Vec<i64>)>) {
    input
        .lines()
        .for_each(|line| {
            let mut equation: (i64, Vec<i64>) = (0, vec![]);
            for (i, part) in line.to_string().split(':').enumerate() {
                if i == 0 {
                    equation.0 = part
                        .to_string()
                        .parse::<i64>()
                        .expect("Parsing error: result")
                } else {
                    equation.1 = part
                        .to_string()
                        .split_whitespace()
                        .map(|c| c
                            .parse::<i64>()
                            .expect("Parsing error: number"))
                        .collect();
                }
            }
            equations.push(equation);
        });
}
