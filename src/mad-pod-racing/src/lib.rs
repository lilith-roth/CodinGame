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

struct PodParameters {
    thrust: i32,
    correction_angle: i32,
    min_correction_speed: i32,
    max_correction_speed: i32,
    multiplier_correction_speed: f32,
    boost_angle: i32,
    boost_distance: i32,
    checkpoint_close_proximity_range: i32,
    checkpoint_close_proximity_correction_angle: i32,
    checkpoint_close_proximity_correction_speed: i32,
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
        let parameters = PodParameters {
            thrust: 100,
            correction_angle: 50,
            min_correction_speed: 25,
            max_correction_speed: 75,
            multiplier_correction_speed: 0.75,
            boost_angle: 30,
            boost_distance: 7000,
            checkpoint_close_proximity_range: 2000,
            checkpoint_close_proximity_correction_angle: 30,
            checkpoint_close_proximity_correction_speed: 15,
        };
        game_loop(
            parameters,
            x,
            y,
            next_checkpoint_x,
            next_checkpoint_y,
            next_checkpoint_dist,
            next_checkpoint_angle,
            opponent_x,
            opponent_y,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn game_loop(
    mut parameters: PodParameters,
    player_x: i32,
    player_y: i32,
    next_checkpoint_x: i32,
    next_checkpoint_y: i32,
    next_checkpoint_dist: i32,
    next_checkpoint_angle: i32,
    opponent_x: i32,
    opponent_y: i32,
) {

    // Thrust adjustments if not facing correct direction
    if next_checkpoint_angle > parameters.correction_angle
        || next_checkpoint_angle < -parameters.correction_angle {
        parameters.thrust = cmp::min(
            cmp::max(
                ((next_checkpoint_angle % 100).abs() as f32
                    * parameters.multiplier_correction_speed).round() as i32,
                parameters.min_correction_speed,
            ),
            parameters.max_correction_speed,
        );
    } else if next_checkpoint_dist >= parameters.boost_distance
        && (next_checkpoint_angle < parameters.boost_angle
        || next_checkpoint_angle > -parameters.boost_angle) {
        // BOOST
        println!("{} {} BOOST", next_checkpoint_x, next_checkpoint_y);
        return;
    }

    // Thrust adjustments if close to checkpoint
    if next_checkpoint_dist < parameters.checkpoint_close_proximity_range {
        // if next_checkpoint_dist < 250 { thrust = 0 } else { thrust = next_checkpoint_dist / 60; }
        // 30 => 16,649
        if next_checkpoint_angle > parameters.checkpoint_close_proximity_correction_angle
            || next_checkpoint_angle < -parameters.checkpoint_close_proximity_correction_angle {
            parameters.thrust = parameters.checkpoint_close_proximity_correction_speed;
        } else {
            parameters.thrust = cmp::max(
                (next_checkpoint_dist & 100).abs(),
                parameters.min_correction_speed,
            );
        }
    }

    // You have to output the target position
    // followed by the power (0 <= thrust <= 100)
    // i.e.: "x y thrust"
    println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, parameters.thrust);
}
