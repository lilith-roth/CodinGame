mod structs;
mod math;
mod utils;

use std::io;

const CHARACTER_MOVE_SPEED: i32 = 1000;
const CHARACTER_KILL_RANGE: i32 = 2000;
const ZOMBIE_MOVE_SPEED: i32 = 400;
const ZOMBIE_KILL_RANGE: i32 = 400;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    let mut last_target: Option<structs::Position> = None;
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let character_position: structs::Position = structs::Position(
            parse_input!(inputs[0], i32),
            parse_input!(inputs[1], i32),
        );
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut humans: Vec<structs::Human> = vec![];
        for _i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            let new_entity = structs::Human {
                entity: structs::Entity {
                    id: human_id,
                    position: structs::Position(human_x, human_y),
                },
                zombies_facing_human: None,
            };
            humans.extend([new_entity]);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        let mut zombies: Vec<structs::Zombie> = vec![];
        for _i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);
            let new_zombie = structs::Zombie {
                entity: structs::Entity {
                    id: zombie_id,
                    position: structs::Position(zombie_x, zombie_y),
                },
                position_next: structs::Position(zombie_xnext, zombie_ynext),
            };
            zombies.extend([new_zombie])
        }

        zombies.sort_by(|a, b|
            math::calculate_distance(b.entity.position, character_position)
                .cmp(&math::calculate_distance(a.entity.position, character_position)));

        let mut target: Option<structs::Position> = None;

        humans.sort_by(|a, b|
            math::calculate_distance(b.entity.position, character_position)
                .cmp(&math::calculate_distance(a.entity.position, character_position)));

        let humans_loop_vec: Vec<structs::Human> = humans.clone();
        'outer: for human in humans_loop_vec.iter() {
            let mut new_human: structs::Human = *human;
            for zombie in zombies.iter() {
                if !zombie.clone().can_character_save_in_time(character_position, new_human.entity.position) {
                    let index = humans.iter().position(|x| x == human).unwrap();
                    humans.remove(index);
                    continue 'outer;
                }

                if zombie.is_zombie_targeting_entity(new_human.entity.position)
                {
                    eprintln!("Potential target {:?}", new_human.entity.position);
                    match new_human.zombies_facing_human {
                        Some(amount) => new_human = {
                            structs::Human {
                                entity: new_human.entity,
                                zombies_facing_human: Option::from(amount + 1),
                            }
                        },
                        None => new_human = structs::Human {
                            entity: new_human.entity,
                            zombies_facing_human: Option::from(1),
                        }
                    }
                }
            }
            let check_human: structs::Human = new_human;
            eprintln!("Facing: {}", check_human.zombies_facing_human.unwrap_or(-1));
            if check_human.zombies_facing_human.unwrap_or(-1) > 0 {
                target = Option::from(check_human.entity.position);
                if Option::from(check_human.entity.position) == last_target {
                    break;
                }
            }
        }
        eprintln!("All grouped {}", utils::are_all_remaining_zombies_grouped(&zombies));
        if zombies.len() == 1 || utils::are_all_remaining_zombies_grouped(&zombies) {
            target = Option::from(zombies[0].entity.position);
        }

        match target {
            Some(position) => {
                eprintln!("Target {:?}", position);
                println!(
                    "{} {}",
                    position.0,
                    position.1
                );
            }
            None => {
                eprintln!("No target!");
                match last_target {
                    Some(position) => {
                        eprintln!("Last Target");
                        target = Option::from(position);
                        println!(
                            "{} {}",
                            position.0,
                            position.1
                        )
                    }
                    None => {
                        eprintln!("No target found! Defaulting");
                        if !humans.is_empty() {
                            target = Option::from(humans[0].entity.position);
                        } else {
                            target = Option::from(zombies[0].entity.position);
                        }
                        println!(
                            "{} {}",
                            target.unwrap().0,
                            target.unwrap().1
                        );
                    }
                }
            }
        }
        last_target = target;
    }
}
