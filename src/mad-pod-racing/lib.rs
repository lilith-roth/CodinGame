use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Clone, Copy)]
struct Checkpoint {
    position_x: i32,
    position_y: i32,
    distance_next_checkpoint: i32,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut last_distance: i32 = 0;
    let mut last_checkpoint_target: Option<Checkpoint> = None;
    let mut checkpoints: Vec<Checkpoint> = vec![];
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
                if checkpoint.position_x != next_checkpoint_x || checkpoint.position_y != next_checkpoint_y {
                    let new_checkpoint = Checkpoint {
                        position_x: next_checkpoint_x,
                        position_y: next_checkpoint_y,
                        distance_next_checkpoint: 0,
                    };
                    last_checkpoint_target = Option::from(new_checkpoint);
                    let already_in_vec = checkpoints.iter().find(|&&search_item| search_item.position_y == next_checkpoint_x && search_item.position_y == next_checkpoint_y);
                    match already_in_vec {
                        None => {
                            checkpoints.extend([new_checkpoint]);
                        }
                        _ => { assert!(false, "Should not happen!"); }
                    }
                }
            }
            None => {
                let checkpoint = Checkpoint {
                    position_x: next_checkpoint_x,
                    position_y: next_checkpoint_y,
                    distance_next_checkpoint: 0,
                };
                last_checkpoint_target = Option::from(checkpoint);
                checkpoints.extend([checkpoint]);
            }
        }
        // if last_checkpoint_target == None {
        //
        // }


        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        let mut thrust = 100;
        if next_checkpoint_dist < 3500 {
            if next_checkpoint_dist < 250 { thrust = 5 } else { thrust = next_checkpoint_dist / 35; }
        } else if next_checkpoint_dist > 5000 {
            // ToDo: Update to only boost on longest straight & disable boost early enough to slow down!
            println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, String::from("BOOST"));
            last_distance = next_checkpoint_dist;
            continue;
        }
        if /*last_distance < next_checkpoint_dist || */next_checkpoint_angle > 80 || next_checkpoint_angle < -80 {
            thrust = (next_checkpoint_angle - 80).abs();
        }
        last_distance = next_checkpoint_dist;
        if thrust > 100 { thrust = 100; }
        println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
    }
}
