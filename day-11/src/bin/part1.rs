

fn main() {
    let input = include_str!("./input.txt");
    let mut vec = gen_vec(input);
    println!("{:?}", vec);
    for _ in 0..25 {
        vec = modify_stones(&mut vec);
    }
    let result = vec.len() as i64;
    println!("Result: {}", result)
}
fn modify_stones(stones: &Vec<i64>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = vec![];
    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
        } else if (*stone).to_string().len() % 2 == 0 {
            let str_stone = (*stone).to_string();
            let (a,b) = str_stone
                .split_at(str_stone.len()/2);
            let a_num = a.parse::<i64>().unwrap();
            let b_num = b.parse::<i64>().unwrap();
            new_stones.push(a_num);
            new_stones.push(b_num);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

fn gen_vec(input: &str) -> Vec<i64> {
    let result = input
        .split_whitespace()
        .map(|s| s
            .parse::<i64>()
            .unwrap())
        .collect::<Vec<i64>>();
    result
}


