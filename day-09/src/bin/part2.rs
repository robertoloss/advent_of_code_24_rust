
fn main() {
    //let input = include_str!("./input1.txt");
    let input = include_str!("./test1.txt");
    let input = input.trim();

    let mut layout = generate_layout(&input);
    println!("Layout: {:?}", formatted(&layout));

    compress_layout(&mut layout);
    println!("Layout: {:?}", formatted(&layout));

    let result = calculate_checksum(&layout);

    println!("Result: {}", result);
}

fn formatted(layout: &Vec<i64>) -> String {
    let mut result: String = "".to_string();
    for num in layout.iter() {
        if *num > -1 {
            result.push_str(&num.to_string());
        } else {
            result.push_str(".");
        }
    }
    result
}

fn generate_layout(input: &str) -> Vec<i64> {
    let mut layout: Vec<i64> = vec![];
    let mut file = 0;
    for (i, c) in input.chars().enumerate() {
        let num = c.to_digit(10).unwrap() as i64;
        if i%2 == 0 {
            for _ in 0..num {
                layout.push(file.clone() as i64);
            }
            file += 1;
        } else {
            for _ in 0..num {
                layout.push(-1)
            }
        }
    }
    layout
}

fn compress_layout(layout: &mut Vec<i64>) {
    let mut right = layout.len() - 1;
    let mut left = 0;

    loop {
        while layout[right] == -1 {
            right -= 1;
        }
        let num = layout[right];
        if num == 0 { 
            break 
        }
        let mut num_length = 0;
        while layout[right] == num {
            num_length += 1;
            right -= 1;
        }
        while layout[left] != num {
            let mut empty_length = 0;
            find_next_empty_space(layout, &mut left, &mut empty_length);
            if empty_length <= num_length {
                todo!();
                break
            }
        }


    }
}
fn find_next_empty_space(
    layout: &Vec<i64>, 
    left: &mut usize,
    empty_length: &mut i64
) {
    while layout[*left] != -1 {
        *left += 1;
    }
    while layout[*left] == -1 {
        *empty_length += 1;
        *left += 1;
    }
}

fn calculate_checksum(layout: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    for (i, num) in layout.iter().enumerate() {
        if *num > -1 {
            result += (i as i64) * num
        }
    }
    result
}
