use std::{cmp, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct GameParameters {
    player_x: i32,
    player_y: i32,
    next_checkpoint_x: i32,
    next_checkpoint_y: i32,
    next_checkpoint_dist: i32,
    next_checkpoint_angle: i32,
    opponent_x: i32,
    opponent_y: i32,
}

struct PodParameters {
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
}

#[derive(Clone, Copy, Debug)]
struct Checkpoint {
    position_x: i32,
    position_y: i32,
    distance_prev_checkpoint: i32,
}

impl Checkpoint {
    fn handle_checkpoint_saving(
        checkpoints: Vec<Checkpoint>,
        mut checkpoints_mapped: bool,
        next_checkpoint_x: i32,
        next_checkpoint_y: i32,
    ) -> (Vec<Checkpoint>, bool) {
        eprintln!("Checkpoints: {:?}", &checkpoints);
        let mut new_checkpoints = checkpoints.to_vec();
        if let Some(checkpoint) = &new_checkpoints.iter()
            .find(|&&x| x.position_x == next_checkpoint_x && x.position_y == next_checkpoint_y) {
            if checkpoints.len() > 1 && !checkpoints_mapped && checkpoint.distance_prev_checkpoint == 0 {
                let prev_checkpoint = &new_checkpoints.last().unwrap();
                let distance = pythagorean_theorem(prev_checkpoint.position_x, prev_checkpoint.position_y);
                new_checkpoints.first_mut().unwrap().distance_prev_checkpoint = distance;
                // If we come here, we're back at the first checkpoint, therefore finalizing mapping.
                checkpoints_mapped = true;
            }
        } else {
            let prev_checkpoint = new_checkpoints.last().unwrap_or(
                // If we want to save the first checkpoint, we'll calculate a distance of 0,
                // and fix that in above's if statement after collecting all checkpoints.
                &Checkpoint {
                    position_x: 0,
                    position_y: 0,
                    distance_prev_checkpoint: 0,
                }
            );
            let distance_prev_checkpoint = pythagorean_theorem(prev_checkpoint.position_x, prev_checkpoint.position_y);
            new_checkpoints.push(Checkpoint {
                position_x: next_checkpoint_x,
                position_y: next_checkpoint_y,
                distance_prev_checkpoint,
            });
        }
        (new_checkpoints, checkpoints_mapped)
    }
}

/// Calculates the pythagorean theorem
/// c^2 = a^2 + b^2 => c = sqrt(a^2 + b^2)
fn pythagorean_theorem(a: i32, b: i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}

fn get_max_distance_checkpoint(checkpoints: &Vec<Checkpoint>) -> Option<&Checkpoint> {
    /*for checkpoint in checkpoints {
        if checkpoint.distance_prev_checkpoint > highest_distance.1 {
            highest_distance = (Some(*checkpoint), checkpoint.distance_prev_checkpoint);
        }
    }*/
    if checkpoints.len() < 2 || checkpoints[0].distance_prev_checkpoint == 0 { return None; }
    let highest_distance = checkpoints.iter().max_by_key(|p| p.distance_prev_checkpoint);
    highest_distance
}

/// Checking if we're on the longest straight to the next checkpoint
///
/// Previous:
/// if game_paramers.next_checkpoint_dist >= pod_parameters.boost_distance
///         && (game_paramers.next_checkpoint_angle < pod_parameters.boost_angle
///         || game_paramers.next_checkpoint_angle > -pod_parameters.boost_angle) {
fn should_boost(
    checkpoints: &Vec<Checkpoint>,
    checkpoints_mapped: bool,
    game_parameters: &GameParameters,
    pod_parameters: &PodParameters,
) -> bool {
    if let Some(checkpoint) = get_max_distance_checkpoint(checkpoints) {
        if checkpoints.len() > 1
            && checkpoints_mapped
            && game_parameters.next_checkpoint_dist > pod_parameters.min_boost_distance
            && (game_parameters.next_checkpoint_angle < pod_parameters.boost_angle || game_parameters.next_checkpoint_angle > -pod_parameters.boost_angle)
            && checkpoint.distance_prev_checkpoint != 0
            && checkpoint.position_x == game_parameters.next_checkpoint_x
            && checkpoint.position_y == game_parameters.next_checkpoint_y {
            return true;
        }
    }
    false
}

fn get_correction_speed(
    game_parameters: &GameParameters,
    pod_parameters: &PodParameters,
) -> i32 {
    cmp::min(
        cmp::max(
            ((game_parameters.next_checkpoint_angle % 100).abs() as f32
                * pod_parameters.multiplier_correction_speed).round() as i32,
            pod_parameters.min_correction_speed,
        ),
        pod_parameters.max_correction_speed,
    )
}

fn get_correction_speed_close_corner(game_parameters: &GameParameters, pod_parameters: &PodParameters) -> i32{
    // if next_checkpoint_dist < 250 { thrust = 0 } else { thrust = next_checkpoint_dist / 60; }
    if game_parameters.next_checkpoint_angle > pod_parameters.checkpoint_close_proximity_correction_angle
        || game_parameters.next_checkpoint_angle < -pod_parameters.checkpoint_close_proximity_correction_angle {
        return pod_parameters.checkpoint_close_proximity_correction_speed;
    }
    cmp::max(
        (game_parameters.next_checkpoint_dist & 100).abs(),
        pod_parameters.min_correction_speed,
    )
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut checkpoints: Vec<Checkpoint> = vec![];
    let mut checkpoints_mapped: bool = false;
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
        let game_parameters = GameParameters {
            player_x: x,
            player_y: y,
            next_checkpoint_x,
            next_checkpoint_y,
            next_checkpoint_dist,
            next_checkpoint_angle,
            opponent_x,
            opponent_y,
        };
        let pod_parameters = PodParameters {
            thrust: 100,
            correction_angle: 45,
            min_correction_speed: 20,
            max_correction_speed: 75,
            multiplier_correction_speed: 0.75,
            boost_angle: 20,
            min_boost_distance: 0,
            checkpoint_close_proximity_range: 2000,
            checkpoint_close_proximity_correction_angle: 25,
            checkpoint_close_proximity_correction_speed: 15,
        };
        let (new_checkpoints, new_checkpoints_mapped) = game_loop(
            game_parameters,
            pod_parameters,
            checkpoints.to_vec(),
            checkpoints_mapped,
        );
        checkpoints = new_checkpoints;
        checkpoints_mapped = new_checkpoints_mapped;
    }
}

/// Handles main game loop
///
/// # Arguments
///
/// * `game_parameters`:
/// * `pod_parameters`:
/// * `previous_checkpoints`:
/// * `checkpoints_mapped`:
///
/// returns: (Vec<Checkpoint, Global>, bool)
fn game_loop(
    game_parameters: GameParameters,
    mut pod_parameters: PodParameters,
    previous_checkpoints: Vec<Checkpoint>,
    checkpoints_mapped: bool,
) -> (Vec<Checkpoint>, bool) {
    eprintln!("Checkpoints mapped {:?}", checkpoints_mapped);
    let (checkpoints, checkpoint_map_status) = Checkpoint::handle_checkpoint_saving(
        previous_checkpoints,
        checkpoints_mapped,
        game_parameters.next_checkpoint_x,
        game_parameters.next_checkpoint_y,
    );
    eprintln!("Checkpoint 0: {:?}", &checkpoints[0].distance_prev_checkpoint);

    // Thrust adjustments if not facing correct direction
    if game_parameters.next_checkpoint_angle > pod_parameters.correction_angle
        || game_parameters.next_checkpoint_angle < -pod_parameters.correction_angle {
        pod_parameters.thrust = get_correction_speed(&game_parameters, &pod_parameters);
    } else if should_boost(&checkpoints, checkpoints_mapped, &game_parameters, &pod_parameters) {
        // BOOST
        eprintln!("BOOST!");
        println!("{} {} BOOST", game_parameters.next_checkpoint_x, game_parameters.next_checkpoint_y);
        return (checkpoints, checkpoint_map_status);
    }

    // Thrust adjustments if close to checkpoint
    if game_parameters.next_checkpoint_dist < pod_parameters.checkpoint_close_proximity_range {
        pod_parameters.thrust = get_correction_speed_close_corner(&game_parameters, &pod_parameters);
    }

    // You have to output the target position
    // followed by the power (0 <= thrust <= 100)
    // i.e.: "x y thrust"
    println!("{} {} {}", game_parameters.next_checkpoint_x, game_parameters.next_checkpoint_y, pod_parameters.thrust);
    (checkpoints, checkpoint_map_status)
}
