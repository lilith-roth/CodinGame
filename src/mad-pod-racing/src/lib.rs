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
        game_loop(
            x,
            y,
            next_checkpoint_x,
            next_checkpoint_y,
            next_checkpoint_dist,
            next_checkpoint_angle,
            opponent_x,
            opponent_y
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn game_loop(
    player_x: i32,
    player_y: i32,
    next_checkpoint_x:i32,
    next_checkpoint_y:i32,
    next_checkpoint_dist: i32,
    next_checkpoint_angle: i32,
    opponent_x: i32,
    opponent_y: i32
) {
    let mut thrust = 100;

    let correction_angle = 50;
    let min_correction_speed = 25;
    let max_correction_speed = 75;
    let multiplier_correction_speed = 0.75;

    let boost_angle = 25; // 20?
    let boost_distance = 7000; // 6500?

    let checkpoint_close_proximity_range = 1500; // 2000?
    let checkpoint_close_proximity_correction_angle = 30;
    let checkpoint_close_proximity_correction_speed = 10;

    // Thrust adjustments if not facing correct direction
    if next_checkpoint_angle > correction_angle
        || next_checkpoint_angle < -correction_angle {
        thrust = cmp::min(
            cmp::max(
                ((next_checkpoint_angle % 100).abs() as f32
                    * multiplier_correction_speed).round() as i32,
                min_correction_speed,
            ),
            max_correction_speed,
        );
    } else if next_checkpoint_dist >= boost_distance
        && (next_checkpoint_angle < boost_angle
        || next_checkpoint_angle > -boost_angle) {
        // BOOST
        println!("{} {} BOOST", next_checkpoint_x, next_checkpoint_y);
        return;
    }

    // Thrust adjustments if close to checkpoint
    if next_checkpoint_dist < checkpoint_close_proximity_range {
        // if next_checkpoint_dist < 250 { thrust = 0 } else { thrust = next_checkpoint_dist / 60; }
        // 30 => 16,649
        if next_checkpoint_angle > checkpoint_close_proximity_correction_angle
            || next_checkpoint_angle < -checkpoint_close_proximity_correction_angle {
            thrust = checkpoint_close_proximity_correction_speed;
        } else {
            thrust = cmp::max(
                (next_checkpoint_dist & 100).abs(),
                min_correction_speed,
            );
        }
    }

    // You have to output the target position
    // followed by the power (0 <= thrust <= 100)
    // i.e.: "x y thrust"
    println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
}
