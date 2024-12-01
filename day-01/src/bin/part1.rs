
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn push_num_to_vec(num: u32, col: &mut Vec<u32>) {
    if col.is_empty() {
        col.push(num)
    } else {
        let mut i = 0;
        while i < col.len() {
            if num < col[i] {
                col.insert(i, num);
                return
            }
            i += 1;
        }
        col.push(num);
    }
}

fn calculate_distance(vec1: Vec<u32>, vec2: Vec<u32>) -> i32 {
    let mut result: i32 = 0;
    for i in 0..vec1.len() {
        let num1 = vec1[i] as i32;
        let num2 = vec2[i] as i32;
        result += (num1 - num2).abs()
    }
    result
}

fn part1(input: &str) -> String {
    let mut first_col: Vec<u32> = vec![];
    let mut second_col: Vec<u32> = vec![];
    input
        .lines()
        .for_each(|line| {
            for (index,item) in line
                .split_whitespace()
                .enumerate() {
                    let num: u32 = item.parse().unwrap();
                    if index == 0 {
                        push_num_to_vec(num, &mut first_col);
                    } else {
                        push_num_to_vec(num, &mut second_col);
                    }
            }
        });
    let result = calculate_distance(first_col, second_col);
    result.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, "11");
    }
}
