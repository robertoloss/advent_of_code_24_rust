
fn main() {
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let mut result: i32 = 0;
    let mut next_char = NextChar::Letter('m');
    let mut first_multiplicand: i32 = 0;
    let mut second_multiplicand: i32 = 0;

    for c in input.chars() {
        match next_char {
            NextChar::Letter(l) => {
                if c == l {
                    match l {
                        'm' => next_char = NextChar::Letter('u'),
                        'u' => next_char = NextChar::Letter('l'),
                        'l' => next_char = NextChar::OpenParenthesis,
                        _ => {}
                    }
                } else { 
                    if l == 'm' { continue }
                    else { reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand) }
                }
            },
            NextChar::OpenParenthesis => {
                if c == '(' {
                    next_char = NextChar::Number((0,"".to_string()))
                } else { reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand) }
            },
            NextChar::Number((count,ref num_string)) => {
                let mut new_string = num_string.clone();
                if c.is_numeric() {
                    new_string.push(c);
                    let new_count = count + 1;
                    next_char = NextChar::Number((new_count, new_string.clone()));
                    if new_count == 3 {
                        if first_multiplicand == 0 {
                            first_multiplicand = new_string.parse::<i32>().unwrap_or(0);
                            next_char = NextChar::Comma;
                        } else {
                            second_multiplicand = new_string.parse::<i32>().unwrap_or(0);
                            next_char = NextChar::ClosedParenthesis;
                        }
                    }
                } else { 
                    match c {
                        ',' => {
                            if first_multiplicand == 0 {
                                first_multiplicand = new_string.parse().unwrap_or(999);
                                next_char = NextChar::Number((0,"".to_string()))
                            } else { 
                                reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand)
                            }
                        },
                        ')' => {
                            if first_multiplicand != 0 && second_multiplicand == 0 {
                                second_multiplicand = new_string.parse::<i32>().unwrap_or(0);
                                result += first_multiplicand * second_multiplicand;
                            }
                            reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand)
                        },
                        _ => reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand)
                    }
                }
            },
            NextChar::Comma => {
                if c == ',' {
                    next_char = NextChar::Number((0,"".to_string()))
                } else { reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand) }
            },
            NextChar::ClosedParenthesis => {
                if c == ')' {
                    result += first_multiplicand * second_multiplicand;
                } 
                reset_next_char(&mut next_char, &mut first_multiplicand, &mut second_multiplicand);
            }
        }
    }
    println!("The result is {}", result)
}

fn reset_next_char(next_char: &mut NextChar, first: &mut i32, second: &mut i32) {
    *next_char = NextChar::Letter('m');
    *first = 0;
    *second = 0;
}
#[derive(Debug)]
enum NextChar {
    Number((u8,String)),
    Letter(char),
    Comma,
    OpenParenthesis,
    ClosedParenthesis
}
