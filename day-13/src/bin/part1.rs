
fn main() {
    let input = include_str!("./test1.txt");
    //let input = include_str!("./input1.txt");
    let mut result: i64 = 0;

    let sections: Vec<&str> = input.split("\n\n").collect();
    
    println!("Result: {}", result);
}
