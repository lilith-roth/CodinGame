use std::io;

const CHARACTER_MOVE_SPEED: i32 = 1000;
const CHARACTER_KILL_RANGE: i32 = 2000;
const ZOMBIE_MOVE_SPEED: i32 = 400;
const ZOMBIE_KILL_RANGE: i32 = 400;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Entity {
    id: i32,
    position: Position,
    is_human: bool,
}

#[derive(Clone, Copy)]
struct Human {
    entity: Entity,
    zombies_facing_human: Option<i32>,
}

#[derive(Clone, Debug)]
struct Zombie {
    entity: Entity,
    position_next: Position,
    distance_to_character: Option<i32>,
    distances_to_humans: Option<Vec<i32>>,
}

impl Zombie {
    fn can_character_save_in_time(self, character_position: Position, target_position: Position) -> bool {
        let target_distance = calculate_distance(self.entity.position, target_position);
        let character_distance = calculate_distance(character_position, target_position);
        let a = target_distance / (ZOMBIE_KILL_RANGE + ZOMBIE_MOVE_SPEED);
        let b = character_distance / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED);
        eprintln!("a {} | b {} | {}", a, b, target_distance / (ZOMBIE_KILL_RANGE + ZOMBIE_MOVE_SPEED) > character_distance / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED));
        target_distance / (ZOMBIE_KILL_RANGE + ZOMBIE_MOVE_SPEED)
            > character_distance / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED)
    }

    fn calculate_danger_level(
        mut self,
        humans: &Vec<Human>,
        character_position: Position,
    ) -> Zombie {
        let distance_x_character: i32 = character_position.0 - self.entity.position.0;
        let distance_y_character: i32 = character_position.0 - self.entity.position.0;
        let distance_character: i32 = pythagorean_theorem(distance_x_character, distance_y_character);
        let mut distances: Vec<i32> = vec![];
        for human in humans.iter() {
            let distance_x_human: i32 =
                human.entity.position.0 - self.position_next.0;
            let distance_y_human: i32 =
                human.entity.position.1 - self.position_next.1;
            let distance_human: i32 = pythagorean_theorem(distance_x_human, distance_y_human);

            if distance_human / (ZOMBIE_KILL_RANGE + ZOMBIE_MOVE_SPEED) > distance_character / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED) {
                distances.extend([distance_human]);
            }
        }
        self.distance_to_character = Option::from(distance_character);
        if distances.is_empty() {
            return self;
        }
        distances.sort();
        self.distances_to_humans = Option::from(distances);
        self
    }

    fn is_zombie_targeting_entity(&self, target_entity_position: Position) -> bool {
        calculate_distance(target_entity_position, self.position_next)
            + calculate_distance(self.entity.position, self.position_next)
            == calculate_distance(target_entity_position, self.entity.position)
    }
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    let mut last_target: Option<Position> = Option::None;
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        // let x = parse_input!(inputs[0], i32);
        // let y = parse_input!(inputs[1], i32);
        let character_position: Position = Position(
            parse_input!(inputs[0], i32),
            parse_input!(inputs[1], i32),
        );
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut humans: Vec<Human> = vec![];
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            let new_entity = Human {
                entity: Entity {
                    id: human_id,
                    position: Position(human_x, human_y),
                    is_human: true,
                },
                zombies_facing_human: Option::None,
            };
            humans.extend([new_entity]);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        let mut zombies: Vec<Zombie> = vec![];
        for i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);
            let new_entity = Zombie {
                entity: Entity {
                    id: zombie_id,
                    position: Position(zombie_x, zombie_y),
                    is_human: false,
                },
                position_next: Position(zombie_xnext, zombie_ynext),
                distance_to_character: Option::None,
                distances_to_humans: Option::None,
            };
            let new_zombie = new_entity.calculate_danger_level(
                &humans,
                Position(
                    character_position.0,
                    character_position.1,
                ),
            );
            zombies.extend([new_zombie])
        }

        zombies.sort_by(|a, b|
            calculate_distance(b.entity.position, character_position)
                .cmp(&calculate_distance(a.entity.position, character_position)));

        let mut target: Option<Position> = None;
        for zombie in zombies.iter() {
            if zombie.is_zombie_targeting_entity(character_position) {
                target = Option::from(zombie.entity.position);
                break;
            }
        }

        // humans.sort_by(|a, b|
        //     calculate_distance(b.entity.position, character_position)
        //     .cmp(&calculate_distance(a.entity.position, character_position)));

        for human in humans.iter() {
            let mut new_human: Human = human.clone();
            for zombie in zombies.iter() {
                // eprintln!("Dist {}", calculate_distance(new_human.entity.position, zombie.entity.position));
                if (zombie.is_zombie_targeting_entity(new_human.entity.position)
                    /*|| is_in_distance(new_human.entity.position, zombie.entity.position, 3000)*/
                    /*|| is_in_distance(character_position, zombie.entity.position, 3000) */)
                    && zombie.clone().can_character_save_in_time(character_position, new_human.entity.position) {
                    eprintln!("Potential target {:?}", new_human.entity.position);
                    match new_human.zombies_facing_human {
                        Some(amount) => new_human = {
                            Human {
                                entity: new_human.entity,
                                zombies_facing_human: Option::from(amount + 1),
                            }
                        },
                        None => new_human = Human {
                            entity: new_human.entity,
                            zombies_facing_human: Option::from(1),
                        }
                    }
                }
            }
            let check_human: Human = new_human.clone();
            eprintln!("Facing: {}", check_human.zombies_facing_human.unwrap_or(-1));
            if check_human.zombies_facing_human.unwrap_or(-1) > 0 {
                target = Option::from(check_human.entity.position);
                if check_human.entity.is_human
                    && Option::from(check_human.entity.position) == last_target {
                    break;
                }
            }
        }
        if zombies.len() == 1 {
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
                match last_target {
                    Some(position) => {
                        target = Option::from(position);
                        println!(
                            "{} {}",
                            position.0,
                            position.1
                        )
                    }
                    None => {
                        let human: &Human = &humans[0];
                        target = Option::from(human.entity.position);
                        println!(
                            "{} {}",
                            human.entity.position.0,
                            human.entity.position.1
                        );
                    }
                }
            }
        }
        last_target = target;

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        // println!(
        //     "{} {}", 
        //     target.entity.position.0, 
        //     target.entity.position.1
        // ); 
        // Your destination coordinates
    }
}

fn pythagorean_theorem(a: i32, b: i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}

fn calculate_distance(a: Position, b: Position) -> i32 {
    ((((a.0 - b.0).pow(2)) + ((a.1 - b.1).pow(2))) as f32).sqrt() as i32
}

fn is_in_distance(a: Position, b: Position, distance: i32) -> bool {
    (distance < calculate_distance(a, b))
}
