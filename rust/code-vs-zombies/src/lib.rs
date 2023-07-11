use std::io;

const CHARACTER_MOVE_SPEED: i32 = 1000;
const CHARACTER_KILL_RANGE: i32 = 2000;
const ZOMBIE_MOVE_SPEED: i32 = 400;
const ZOMBIE_KILL_RANGE: i32 = 400;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Position(i32, i32);

struct Entity {
    id: i32,
    position: Position
}

struct Zombie {
    entity: Entity,
    position_next: Position,
    distance_to_character: Option<i32>,
    distances_to_humans: Option<Vec<i32>>
}

impl Zombie {
    fn calculate_danger_level(
        mut self, 
        humans: &Vec<Entity>,
        character_position: Position
    ) -> Zombie {
        let distance_x_character: i32 = character_position.0 - self.entity.position.0;
        let distance_y_character: i32 = character_position.0 - self.entity.position.0;
        let distance_character: i32 = pythagorean_theorem(distance_x_character, distance_y_character);
        let mut distances: Vec<i32> = vec![];
        for human in humans.iter() {
            let distance_x_human: i32 = human.position.0 - self.entity.position.0;
            let distance_y_human: i32 = human.position.1 - self.entity.position.1;
            let distance_human: i32 = pythagorean_theorem(distance_x_human, distance_y_human);
            
            eprintln!("dH {}", distance_human / 400);
            eprintln!("cH {}", distance_character / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED));
            if distance_human / (ZOMBIE_KILL_RANGE+ZOMBIE_MOVE_SPEED) > distance_character / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED) {
                distances.extend([distance_human]);
            }
        }
        distances.sort();
        self.distances_to_humans = Option::from(distances);
        self.distance_to_character = Option::from(distance_character);
        self
    }
}

// fn get_next_zombie_to_kill(
//     character: Position,
//     humans: Vec<Entity>, 
//     zombies: Vec<Zombie>
// ) -> Zombie {
//     for zombie in zombies.iter() {
//         let distance_to_zombie: i32 = pythagorean_theorem(
//             character.0 - zombie.entity.position.0, 
//             character.1 - zombie.entity.position.1
//         );
//         let zombie_distances_to_humans: Vec<i32> = zombie.distances_to_humans.unwrap_or(vec![i32::MAX]);
//         for distance in zombie_distances_to_humans.iter() {

//         }
//     }
// }

/**
 * Save humans, destroy zombies!
 **/
fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut humans: Vec<Entity> = vec![];
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            let new_entity = Entity {
                id: human_id,
                position: Position(human_x, human_y)
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
                    position: Position(zombie_x, zombie_y)
                },
                position_next: Position(zombie_xnext, zombie_ynext),
                distance_to_character: Option::None,
                distances_to_humans: Option::None
            };
            let new_zombie = new_entity.calculate_danger_level(
                &humans,
                Position(x, y)
            );
            zombies.extend([new_zombie])
        }
        zombies.sort_by(|a,b| {
            let a_human_dist = a.distances_to_humans.as_ref().unwrap_or(&vec![i32::MAX])[0];
            let b_human_dist = b.distances_to_humans.as_ref().unwrap_or(&vec![i32::MAX])[0];
            if (a_human_dist == b_human_dist) {
                a.distance_to_character.cmp(&b.distance_to_character)
            }
            else
            {
                a_human_dist.cmp(&b_human_dist)
            }
        });

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        println!(
            "{} {}", 
            zombies[0].entity.position.0, 
            zombies[0].entity.position.1
        ); // Your destination coordinates
    }
}

fn pythagorean_theorem(a: i32, b:i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}
