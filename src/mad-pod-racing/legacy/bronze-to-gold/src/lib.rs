use std::{cmp, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct GameParameters {
    player_position: Position,
    opponent_position: Position,
    next_checkpoint_x: i32,
    next_checkpoint_y: i32,
    next_checkpoint_dist: i32,
    next_checkpoint_angle: i32,
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
    checkpoint_target_offset_multiplier: i32,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    position_x: i32,
    position_y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Checkpoint {
    position: Position,
    distance_prev_checkpoint: i32,
}

impl Checkpoint {
    /// Makes sure we're saving all checkpoints in a Vector.
    ///
    /// While saving a new checkpoint, we also determine the distance to the previous one.
    /// This is used to determine the longest straight on which we'll boost.
    ///
    /// When saving the first checkpoint we don't have the last one saved yet, and are therefore
    /// unable to determine the distance.
    /// For that reason we're calculating this distance on start of turn 2.
    fn handle_checkpoint_saving(
        checkpoints: Vec<Checkpoint>,
        mut checkpoints_mapped: bool,
        next_checkpoint_x: i32,
        next_checkpoint_y: i32,
    ) -> (Vec<Checkpoint>, bool) {
        eprintln!("Checkpoints: {:?}", &checkpoints);
        let mut new_checkpoints = checkpoints.to_vec();
        if let Some(checkpoint) = &new_checkpoints.iter()
            .find(|&&x| x.position.position_x == next_checkpoint_x && x.position.position_y == next_checkpoint_y) {
            if checkpoints.len() > 1 && !checkpoints_mapped && checkpoint.distance_prev_checkpoint == 0 {
                let prev_checkpoint = &new_checkpoints.last().unwrap();
                let distance = pythagorean_theorem(prev_checkpoint.position.position_x, prev_checkpoint.position.position_y);
                new_checkpoints.first_mut().unwrap().distance_prev_checkpoint = distance;
                // If we come here, we're back at the first checkpoint, therefore finalizing mapping.
                checkpoints_mapped = true;
            }
        } else {
            let prev_checkpoint = new_checkpoints.last().unwrap_or(
                // If we want to save the first checkpoint, we'll calculate a distance of 0,
                // and fix that in above's if statement after collecting all checkpoints.
                &Checkpoint {
                    position: Position {
                        position_x: 0,
                        position_y: 0,
                    },
                    distance_prev_checkpoint: 0,
                }
            );
            let distance_prev_checkpoint = pythagorean_theorem(prev_checkpoint.position.position_x, prev_checkpoint.position.position_y);
            new_checkpoints.push(Checkpoint {
                position: Position {
                    position_x: next_checkpoint_x,
                    position_y: next_checkpoint_y,
                },
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

/// Determines the checkpoint with the highest distance to the previous one
fn get_max_distance_checkpoint(checkpoints: &Vec<Checkpoint>) -> Option<&Checkpoint> {
    if checkpoints.len() < 2 || checkpoints[0].distance_prev_checkpoint == 0 { return None; }
    let highest_distance = checkpoints.iter().max_by_key(|p| p.distance_prev_checkpoint);
    highest_distance
}

/// Checking if we're on the longest straight to the next checkpoint
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
            && game_parameters.next_checkpoint_angle < pod_parameters.boost_angle
            && game_parameters.next_checkpoint_angle > -pod_parameters.boost_angle
            && checkpoint.distance_prev_checkpoint != 0
            && checkpoint.position.position_x == game_parameters.next_checkpoint_x
            && checkpoint.position.position_y == game_parameters.next_checkpoint_y {
            return true;
        }
    }
    false
}

/// Function that determines the speed used while adjusting rotation
///
/// ToDo: Currently used as a general speed function, and works well this way.
///       Though i think we can get more out of it by just using this either:
///         - Below a certain distance before reaching the next checkpoint.
///         - While outside of a certain angle range in relation to the next checkpoint.
fn get_correction_speed(
    game_parameters: &GameParameters,
    pod_parameters: &PodParameters,
) -> i32 {
    let mut speed =
        (((1 - game_parameters.next_checkpoint_angle / 90) as f32)
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

/// Used to further slow down if close to a corner.
///
/// ToDo: May be unnecessary now with above's speed function being able to handle this.
fn get_correction_speed_close_corner(game_parameters: &GameParameters, pod_parameters: &PodParameters) -> i32 {
    if game_parameters.next_checkpoint_angle > pod_parameters.checkpoint_close_proximity_correction_angle
        || game_parameters.next_checkpoint_angle < -pod_parameters.checkpoint_close_proximity_correction_angle {
        return pod_parameters.checkpoint_close_proximity_correction_speed;
    }
    cmp::max(
        (game_parameters.next_checkpoint_dist & 100).abs(),
        pod_parameters.min_correction_speed,
    )
}

/// Determines the speed of the pod in the current epoch.
fn get_pod_speed(game_parameters: &GameParameters, last_pod_position: (i32, i32)) -> (i32, i32) {
    let delta_x = game_parameters.player_position.position_x - last_pod_position.0;
    let delta_y = game_parameters.player_position.position_y - last_pod_position.1;
    eprintln!("Speed: {:?}", (delta_x, delta_y));
    (delta_x, delta_y)
}

/// Determines the target to aim for with regard to the current pods speed.
fn get_target_coordinates(
    game_parameters: &GameParameters,
    pod_parameters: &PodParameters,
    last_pod_position: (i32, i32),
) -> (i32, i32) {
    let pod_speed = get_pod_speed(
        game_parameters,
        last_pod_position,
    );
    (
        (
            game_parameters.next_checkpoint_x +
                (- pod_parameters.checkpoint_target_offset_multiplier
                * pod_speed.0)
        ),
        (
            game_parameters.next_checkpoint_y +
                (- pod_parameters.checkpoint_target_offset_multiplier
                * pod_speed.1)
        ),
    )
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut checkpoints: Vec<Checkpoint> = vec![];
    let mut checkpoints_mapped: bool = false;
    let mut last_pod_position: (i32, i32) = (0, 0);
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
        if last_pod_position == (0,0) {
            last_pod_position = (x, y)
        };
        let game_parameters = GameParameters {
            player_position: Position {
                position_x: x,
                position_y: y,
            },
            opponent_position: Position{
                position_x: opponent_x,
                position_y: opponent_y
            },
            next_checkpoint_x,
            next_checkpoint_y,
            next_checkpoint_dist,
            next_checkpoint_angle,
        };
        let pod_parameters = PodParameters {
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
        let (new_checkpoints, new_checkpoints_mapped) = game_loop(
            game_parameters,
            pod_parameters,
            checkpoints.to_vec(),
            checkpoints_mapped,
            last_pod_position,
        );
        checkpoints = new_checkpoints;
        checkpoints_mapped = new_checkpoints_mapped;
        last_pod_position = (x, y);
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
    last_pod_position: (i32, i32),
) -> (Vec<Checkpoint>, bool) {
    eprintln!("Checkpoints mapped {:?}", checkpoints_mapped);
    let (checkpoints, checkpoint_map_status) = Checkpoint::handle_checkpoint_saving(
        previous_checkpoints,
        checkpoints_mapped,
        game_parameters.next_checkpoint_x,
        game_parameters.next_checkpoint_y,
    );

    // Thrust adjustments if not facing correct direction
    /*if game_parameters.next_checkpoint_angle > pod_parameters.correction_angle
        || game_parameters.next_checkpoint_angle < -pod_parameters.correction_angle {*/
        pod_parameters.thrust = get_correction_speed(&game_parameters, &pod_parameters);
    /*} else*/ if should_boost(&checkpoints, checkpoints_mapped, &game_parameters, &pod_parameters) {
        // BOOST
        eprintln!("BOOST!");
        println!("{} {} BOOST", game_parameters.next_checkpoint_x, game_parameters.next_checkpoint_y);
        return (checkpoints, checkpoint_map_status);
    }

    // Thrust adjustments if close to checkpoint
    if game_parameters.next_checkpoint_dist < pod_parameters.checkpoint_close_proximity_range {
        pod_parameters.thrust = get_correction_speed_close_corner(&game_parameters, &pod_parameters);
    }

    let (target_x, target_y) = get_target_coordinates(
        &game_parameters,
        &pod_parameters,
        last_pod_position,
    );

    // You have to output the target position
    // followed by the power (0 <= thrust <= 100)
    // i.e.: "x y thrust"
    println!(
        "{} {} {}",
        target_x,
        target_y,
        pod_parameters.thrust
    );
    (checkpoints, checkpoint_map_status)
}
