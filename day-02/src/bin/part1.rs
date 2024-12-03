


fn main() {
    let input = include_str!("./input1.txt");
    //let input = include_str!("./test1.txt");
    //println!("{}", input);
    let mut result: i32 = 0;
    input
        .lines()
        .for_each(|report| {
            let mut increasing: i32 = 0;
            let mut decreasing: i32 = 0;
            let mut difference_ok = true;
            let report_vec: &Vec<i32> = &report
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            for pair in report_vec.windows(2) {
                let difference = pair[1] - pair[0];
                if difference.abs() < 1 || difference.abs() > 3 { 
                    difference_ok = false ;
                    break
                }
                if difference > 0 {
                    increasing += 1
                } else {
                    decreasing += 1
                }
            }
            if difference_ok && !(increasing != 0 && decreasing !=0) {
                result += 1
            }
        });

    println!("{}",result);
}
