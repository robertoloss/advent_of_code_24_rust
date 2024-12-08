use std::collections::HashMap;


fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let mut result: i64 = 0;

    let parts: Vec<&str> = input
        .split("\n\n")
        .collect();

    let instructions = parts[0];
    let updates = parts[1];

    let mut instruction_map = HashMap::<i64, Vec<i64>>::new();

    generate_instruction_map(&mut instruction_map, instructions);

    for update in updates.lines() {
        result += update_is_ok(update, &instruction_map)
    }
    
    println!("Result: {}", result);
}

fn update_is_ok(update: &str, instruction_map: &HashMap<i64, Vec<i64>>) -> i64 {
    let update_vec: Vec<i64> = update
        .split(',')
        .map(|c| c.parse::<i64>().unwrap())
        .collect();
    for i in 0..update_vec.len() {
        if i == 0 { continue }
        if instruction_map.get(&update_vec[i]).is_some() {
            let values = instruction_map.get(&update_vec[i]).unwrap();
            let mut j = i as i8;
            while j >= 0 {
                if values.contains(&update_vec[j as usize]) { 
                    return 0 
                }
                j -= 1;
            }
        }
    }
    let m = ((update_vec.len() as f64 /2.0).floor()) as usize;
    update_vec[m]
}

fn generate_instruction_map(
    instruction_map: &mut HashMap<i64, Vec<i64>>,
    instructions: &str

) {
    for instruction in instructions.lines() {
        let parts: Vec<&str> = instruction
            .split('|')
            .collect();

        let first = parts[0].parse::<i64>().unwrap();
        let second = parts[1].parse::<i64>().unwrap();

        if instruction_map.get(&first).is_some() {
            let vec = instruction_map.get_mut(&first).unwrap();
            (*vec).push(second);
        } else {
            instruction_map.insert(first, vec![second]);
        }
    }
}
