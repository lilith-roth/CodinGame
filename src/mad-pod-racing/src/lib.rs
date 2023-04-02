use std::{cmp, io, num};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Clone, Copy, Debug)]
struct Checkpoint {
    position_x: i32,
    position_y: i32,
    distance_last_checkpoint: i32,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        eprintln!("input: {:?}", inputs);
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
        let next_checkpoint_dist = parse_input!(inputs[4], i32); // distance to the next checkpoint
        let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_x = parse_input!(inputs[0], i32);
        let opponent_y = parse_input!(inputs[1], i32);
        eprintln!("enemy_input: {:?}", inputs);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        let mut thrust = 100;
        let min_speed = 25;

        if next_checkpoint_angle != 0
            && (next_checkpoint_angle > 45 || next_checkpoint_angle < -45) {
            thrust = cmp::max(next_checkpoint_angle % 100, min_speed);
        } else if next_checkpoint_dist >= 6000
            && (next_checkpoint_angle < 45 || next_checkpoint_angle > -45) {
            // BOOST!
            println!("{} {} BOOST", next_checkpoint_x, next_checkpoint_y);
            continue;
        }
        if next_checkpoint_dist < 2000 {
            // if next_checkpoint_dist < 250 { thrust = 0 } else { thrust = next_checkpoint_dist / 60; }
            if next_checkpoint_angle > 30 {
                thrust = 5;
            } else {
                thrust = cmp::max(100 % next_checkpoint_dist, min_speed);
            }
        }

        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
    }
}
