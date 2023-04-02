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
    let mut last_distance: i32 = 0;
    let mut last_checkpoint_target: Option<Checkpoint> = None;
    let mut checkpoints: Vec<Checkpoint> = vec![];
    let mut all_checkpoints_saved = false;
    let mut distances_calculated = false;

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
        if last_distance == 0 { last_distance = next_checkpoint_dist; }
        match &last_checkpoint_target {
            Some(checkpoint) => {
                if checkpoint.position_x != next_checkpoint_x
                    || checkpoint.position_y != next_checkpoint_y {
                    let new_checkpoint = Checkpoint {
                        position_x: next_checkpoint_x,
                        position_y: next_checkpoint_y,
                        distance_last_checkpoint: 0,
                    };
                    last_checkpoint_target = Option::from(new_checkpoint);

                    if all_checkpoints_saved {
                        eprintln!("ALL CHECKPOINTS SAVED!");
                        // If we have all checkpoints saved but no distances yet, calculate them here.
                        if !distances_calculated {
                            let length = &checkpoints.len();
                            let checkpoints_temp = checkpoints.to_vec();
                            for (i, checkpoint) in checkpoints.iter_mut().enumerate() {
                                if i == 0 { continue; }
                                if checkpoint.distance_last_checkpoint == 0 {
                                    let distance_x = checkpoints_temp[i - 1].position_x - checkpoint.position_x;
                                    let distance_y = checkpoints_temp[i - 1].position_y - checkpoint.position_y;
                                    let distance = pythagorean_theorem(distance_x, distance_y);
                                    checkpoint.distance_last_checkpoint = distance;
                                }
                            }
                            distances_calculated = true;
                        }
                    } else {
                        // If we don't have all checkpoints saved yet, add next one to our list.
                        let already_in_vec = checkpoints.iter().find(|&&search_item| search_item.position_y == next_checkpoint_x && search_item.position_y == next_checkpoint_y);
                        match already_in_vec {
                            None => {
                                checkpoints.extend([new_checkpoint]);
                            }
                            _ => { unreachable!("Should not happen!"); }
                        }
                    }
                } else {
                    all_checkpoints_saved = true;
                }
            }
            None => {
                let checkpoint = Checkpoint {
                    position_x: next_checkpoint_x,
                    position_y: next_checkpoint_y,
                    distance_last_checkpoint: 0,
                };
                last_checkpoint_target = Option::from(checkpoint);
                checkpoints.extend([checkpoint]);
            }
        }

        let mut thrust = 100;
        if next_checkpoint_dist < 2000 {
            // if next_checkpoint_dist < 250 { thrust = 0 } else { thrust = next_checkpoint_dist / 60; }
            thrust = cmp::max(next_checkpoint_dist / 200, 10);
        }

        if distances_calculated {
            let max_dist_checkpoint = get_max_distance_checkpoint(&checkpoints);
            eprintln!("max_dist: {:?}", max_dist_checkpoint);
            if let Some(checkpoint) = max_dist_checkpoint {
                if checkpoint.position_x == next_checkpoint_x
                    && checkpoint.position_y == next_checkpoint_y {
                    println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, String::from("BOOST"));
                    last_distance = next_checkpoint_dist;
                    continue;
                }
            }
        }
        if /*last_distance < next_checkpoint_dist || */next_checkpoint_angle > 45 || next_checkpoint_angle < -45 {
            thrust = (next_checkpoint_angle - 100).abs() / 4;
        }

        last_distance = next_checkpoint_dist;
        if thrust > 100 { thrust = 100; } // ToDo: Figure out why we sometimes get thrust > 100!

        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
    }
}

fn get_max_distance_checkpoint(checkpoints: &Vec<Checkpoint>) -> Option<Checkpoint> {
    let mut highest_distance: (Option<Checkpoint>, i32) = (None, 0);
    for checkpoint in checkpoints {
        if checkpoint.distance_last_checkpoint > highest_distance.1 {
            highest_distance = (Some(*checkpoint), checkpoint.distance_last_checkpoint);
        }
    }
    highest_distance.0
}

/// Calculates the pythagorean theorem
/// c^2 = a^2 + b^2 => c = sqrt(a^2 + b^2)
fn pythagorean_theorem(a: i32, b: i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32;
}
