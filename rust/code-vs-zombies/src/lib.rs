use std::io;

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
    danger_level: Option<i32>
}

impl Zombie {
    fn calculate_danger_level(mut self) -> Zombie {
        let danger_level: i32 = 0;
        self.danger_level = Option::from(danger_level);
        self
    }
}

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
                danger_level: Option::None
            };
            let new_zombie = new_entity.calculate_danger_level();
            zombies.extend([new_zombie])
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        println!(
            "{} {}", 
            humans[0].position.0, 
            humans[0].position.1
        ); // Your destination coordinates
    }
}

fn pythagorean_theorem(a: i32, b:i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}
