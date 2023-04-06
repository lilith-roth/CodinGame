use std::{cmp, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct GameInput {
    player_position: Position,
    speed: Speed,
    next_checkpoint_id: i32,
    next_checkpoint_dist: i32,
    next_checkpoint_angle: i32,
}

struct Pod {
    thrust: i32,
    correction_angle: i32,
    min_correction_speed: i32,
    max_correction_speed: i32,
    multiplier_correction_speed: f32,
    boost_angle: i32,
    min_boost_distance: i32,
    checkpoint_close_proximity_range: i32,
    checkpoint_close_proximity_correction_angle: i32,
    checkpoint_close_proximity_correction_speed: i32,
    checkpoint_target_offset_multiplier: i32,
}

struct Speed(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Checkpoint {
    checkpoint_id: i32,
    position: Position,
    distance_prev_checkpoint: Option<i32>,
}

impl GameInput {
    /// Determines the target to aim for with regard to the current pods speed.
    fn get_target_coordinates(
        &self,
        pod_parameters: &Pod,
        pod_speed: &Speed,
        next_checkpoint: &Checkpoint
    ) -> (i32, i32) {
        (
            (
                next_checkpoint.position.0 +
                    (-pod_parameters.checkpoint_target_offset_multiplier
                        * pod_speed.0)
            ),
            (
                next_checkpoint.position.1 +
                    (-pod_parameters.checkpoint_target_offset_multiplier
                        * pod_speed.1)
            ),
        )
    }

    /// Checking if we're on the longest straight to the next checkpoint
    fn should_boost(
        &self,
        pod_parameters: &Pod,
        next_checkpoint: &Checkpoint
    ) -> bool {
        if self.next_checkpoint_dist > pod_parameters.min_boost_distance
            && self.next_checkpoint_angle < pod_parameters.boost_angle
            && self.next_checkpoint_angle > -pod_parameters.boost_angle
            && next_checkpoint.distance_prev_checkpoint.unwrap() != 0 // ToDo: Unwrap!
            && next_checkpoint.checkpoint_id == self.next_checkpoint_id {
            return true;
        }
        false
    }

    /// Function that determines the speed used while adjusting rotation
    fn get_target_speed(
        &self,
        pod_parameters: &Pod,
    ) -> i32 {
        let mut speed =
            (((1 - self.next_checkpoint_angle / 90) as f32)
                * 100_f32)
                .round() as i32;
        // ToDo: Figure out why above calculation sometimes returns 200
        if speed == 200 {
            speed = 100;
        }
        cmp::max(
            cmp::min(
                (speed as f32 * pod_parameters.multiplier_correction_speed) as i32,
                pod_parameters.min_correction_speed,
            ),
            pod_parameters.max_correction_speed,// keep?
        )
    }
}

/// Calculates the pythagorean theorem
/// c^2 = a^2 + b^2 => c = sqrt(a^2 + b^2)
fn pythagorean_theorem(a: i32, b: i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}

/// Determines the checkpoint with the highest distance to the previous one
fn get_max_distance_checkpoint(checkpoints: &Vec<Checkpoint>) -> Option<&Checkpoint> {
    if checkpoints.len() < 2 || checkpoints[0].distance_prev_checkpoint.unwrap() == 0 { return None; } // ToDo: Unwrap!
    let highest_distance = checkpoints.iter().max_by_key(|p| p.distance_prev_checkpoint);
    highest_distance
}

/// Determines the speed of the pod in the current epoch.
fn get_pod_speed(game_parameters: &GameInput, last_pod_position: (i32, i32)) -> (i32, i32) {
    let delta_x = game_parameters.player_position.0 - last_pod_position.0;
    let delta_y = game_parameters.player_position.1 - last_pod_position.1;
    eprintln!("Speed: {:?}", (delta_x, delta_y));
    (delta_x, delta_y)
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let laps = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let checkpoint_count = parse_input!(input_line, i32);
    let mut checkpoints: Vec<Checkpoint> = vec![];
    for i in 0..checkpoint_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let checkpoint_x = parse_input!(inputs[0], i32);
        let checkpoint_y = parse_input!(inputs[1], i32);
        if i == 0 {
            checkpoints.extend([Checkpoint {
                checkpoint_id: i as i32,
                position: Position(checkpoint_x, checkpoint_y),
                distance_prev_checkpoint: None,
            }]);
        } else {
            checkpoints.extend([Checkpoint {
                checkpoint_id: i as i32,
                position: Position(checkpoint_x, checkpoint_y),
                distance_prev_checkpoint: Option::from(pythagorean_theorem(checkpoints[i - 1].position.0, checkpoints[i - 1].position.1)),
            }]);
        }
    }
    checkpoints[0].distance_prev_checkpoint = Option::from(pythagorean_theorem(
        (checkpoints[0].position.0 - checkpoints[checkpoints.len() - 1].position.0).abs(),
        (checkpoints[0].position.1 - checkpoints[checkpoints.len() - 1].position.1)).abs()
    );

    // game loop
    loop {
        let mut pod_inputs: Vec<GameInput> = vec![];
        let mut enemy_pod_inputs: Vec<GameInput> = vec![];
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32); // x position of your pod
            let y = parse_input!(inputs[1], i32); // y position of your pod
            let vx = parse_input!(inputs[2], i32); // x speed of your pod
            let vy = parse_input!(inputs[3], i32); // y speed of your pod
            let angle = parse_input!(inputs[4], i32); // angle of your pod
            let next_check_point_id = parse_input!(inputs[5], i32); // next check point id of your pod
            eprintln!("input {} {:?}", i, inputs);

            let target_checkpoint = checkpoints.get(next_check_point_id as usize);
            let pod_input = GameInput {
                player_position: Position(x, y),
                speed: Speed(vx, vy),
                next_checkpoint_id: next_check_point_id,
                next_checkpoint_dist: target_checkpoint.unwrap().distance_prev_checkpoint.unwrap(), // ToDo: Unwrap!
                next_checkpoint_angle: angle, // ToDo: Calc!
            };
            pod_inputs.extend([pod_input]);
        }
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x_2 = parse_input!(inputs[0], i32); // x position of the opponent's pod
            let y_2 = parse_input!(inputs[1], i32); // y position of the opponent's pod
            let vx_2 = parse_input!(inputs[2], i32); // x speed of the opponent's pod
            let vy_2 = parse_input!(inputs[3], i32); // y speed of the opponent's pod
            let angle_2 = parse_input!(inputs[4], i32); // angle of the opponent's pod
            let next_check_point_id_2 = parse_input!(inputs[5], i32); // next check point id of the opponent's pod
            eprintln!("input enemy {} {:?}", i, inputs);

            let target_checkpoint = checkpoints.get(next_check_point_id_2 as usize);
            let pod_input = GameInput {
                player_position: Position(x_2, y_2),
                speed: Speed(vx_2, vy_2),
                next_checkpoint_id: next_check_point_id_2,
                next_checkpoint_dist: target_checkpoint.unwrap().distance_prev_checkpoint.unwrap(), // ToDo: Unwrap!
                next_checkpoint_angle: angle_2, // ToDo: Calc!
            };
            enemy_pod_inputs.extend([pod_input]);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        for pod in pod_inputs.iter() {
            let pod_parameters = Pod {
                thrust: 100,
                correction_angle: 45,
                min_correction_speed: 20,
                max_correction_speed: 100,
                multiplier_correction_speed: 0.5,
                boost_angle: 20,
                min_boost_distance: 0,
                checkpoint_close_proximity_range: 2000,
                checkpoint_close_proximity_correction_angle: 25,
                checkpoint_close_proximity_correction_speed: 15,
                checkpoint_target_offset_multiplier: 3,
            };
            let next_checkpoint = checkpoints.get(pod.next_checkpoint_id as usize);

            let (target_x, target_y) = pod.get_target_coordinates(
                &pod_parameters,
                &pod.speed,
                next_checkpoint.unwrap() // ToDo: Unwrap!
            );
            let thrust = pod.get_target_speed(&pod_parameters);
            // BOOST
            if pod.should_boost(&pod_parameters, next_checkpoint.unwrap()) { // ToDo: Unwrap!
                println!("{} {} BOOST", target_x, target_y);
                continue;
            }
            println!(
                "{} {} {}",
                target_x,
                target_y,
                thrust
            );
        }
    }
}


