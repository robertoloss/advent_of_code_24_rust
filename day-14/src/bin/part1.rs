use std::{process::Command, usize};

#[derive(Debug)]
struct Vec2 {
    x: i64,
    y: i64
}
#[derive(Debug)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

fn main() {
    Command::new("clear").status().unwrap();
    //let input = include_str!("./test1.txt");
    let input = include_str!("./input1.txt");
    let mut result: u64 = 1;
    let mut robots: Vec<Robot> = generate_robots(&input);
    let seconds: usize = 100;
    let room_dimensions = Vec2 {
        x: 101,
        y: 103
    };
    let quad_dimensions = Vec2 {
        x: (room_dimensions.x / 2),
        y: (room_dimensions.y / 2)
    };
    let mut quads_nums: Vec<u64> = [0,0,0,0].to_vec();

    for _ in 0..seconds {
        for robot in &mut robots {
            move_robot(robot, &room_dimensions);
        }
    }
    for robot in &robots {
        if robot.pos.x != quad_dimensions.x &&
            robot.pos.y != quad_dimensions.y 
        {
            if robot.pos.x < quad_dimensions.x {
                if robot.pos.y < quad_dimensions.y {
                    quads_nums[0] += 1
                } else {
                    quads_nums[2] += 1
                }
            } else {
                if robot.pos.y < quad_dimensions.y {
                    quads_nums[1] += 1
                } else {
                    quads_nums[3] += 1
                }
            }
        }
    }
    println!("\n");
    for y in 0..room_dimensions.y {
        let mut line: String = "".to_string();
        for x in 0..room_dimensions.x {
            if x != quad_dimensions.x && y != quad_dimensions.y {
                let mut num_of_robots = 0;
                for robot in &robots {
                    if robot.pos.x == x && robot.pos.y == y {
                        num_of_robots += 1
                    }
                }
                if num_of_robots > 0 {
                    line.push_str(&num_of_robots.to_string());
                } else {
                    line.push('.')
                }
            } else {
                line.push(' ')
            }
        }
        println!("{}", line)
    }
    for num in &quads_nums {
        result *= num
    }
    println!("\nRoom dimensions: {:?}", room_dimensions);
    println!();
    for robot in &robots {
        println!("{:?}", robot);
    }
    println!("\n\nQuad nums: {:?}", quads_nums);
    println!("Quad dimensions: {:?}", quad_dimensions);
    println!("Result: {}", result);
}

fn move_robot(robot: &mut Robot, room_dimensions: &Vec2) {
    let mut new_pos = Vec2 {
        x: robot.pos.x + robot.vel.x,
        y: robot.pos.y + robot.vel.y
    };
    if new_pos.x < 0 {
        let add = if new_pos.x % room_dimensions.x == 0 { -room_dimensions.x } else { new_pos.x % room_dimensions.x };
        new_pos.x = room_dimensions.x + add;
    }
    if new_pos.x > room_dimensions.x - 1 {
        new_pos.x = new_pos.x % room_dimensions.x
    }
    if new_pos.y < 0 {
        let add = if new_pos.y % room_dimensions.y == 0 { -room_dimensions.y } else { new_pos.y % room_dimensions.y };
        new_pos.y = room_dimensions.y + add
    }
    if new_pos.y > room_dimensions.y - 1 {
        new_pos.y = new_pos.y % room_dimensions.y
    }
    robot.pos.x = new_pos.x;
    robot.pos.y = new_pos.y;
}


fn generate_robots(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];
    let mut num_robots = 0;
    for line in input.lines() {
        num_robots += 1;
        let mut robot = Robot {
            pos: Vec2 {
                x: 0,
                y: 0
            },
            vel: Vec2 {
                x: 0,
                y: 0
            }
        };
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut parts_parsed: Vec<Vec<i64>> = vec![];
        for part in parts {
            let nums_str = part.split('=').nth(1).unwrap();
            let nums: Vec<i64> = nums_str
                .split(',')
                .map(|s| {
                    s.parse::<i64>().unwrap()
                })
                .collect();
            parts_parsed.push(nums)
        }
        robot.pos.x = parts_parsed[0][0];
        robot.pos.y = parts_parsed[0][1];
        robot.vel.x = parts_parsed[1][0];
        robot.vel.y = parts_parsed[1][1];
        robots.push(robot);
    }
    println!("Number of robots: {}", num_robots);
    robots
}
