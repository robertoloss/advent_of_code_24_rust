
fn main() {
    let input = include_str!("./input1.txt");
    //let input = include_str!("./test1.txt");
    let input = input.trim();

    let mut layout = generate_layout(&input);

    compress_layout(&mut layout);

    let result = calculate_checksum(&layout);

    println!("Result: {}", result);
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
    let mut i = 0;
    let mut end = layout.len() - 1;
    while i < end {
        if layout[i] < 0 {
            while !layout[end] > -1 {
                end -= 1
            }
            let num = layout[end].clone();
            let _ = std::mem::replace(&mut layout[i], num);
            let _ = std::mem::replace(&mut layout[end], -1);
            end -= 1;
        }
        i += 1
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
