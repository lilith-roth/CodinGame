use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
struct Mountain(usize, i32);

/**
 * The while loop represents the game.
 * Each iteration represents a turn of the game
 * where you are given inputs (the heights of the mountains)
 * and where you have to print an output (the index of the mountain to fire on)
 * The inputs you are given are automatically updated according to your last actions.
 **/
fn main() {
    // game loop
    loop {
        let mut mountains: Vec<Mountain> = vec![];
        for i in 0..8 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let mountain_h = parse_input!(input_line, i32); // represents the height of one mountain.
            mountains.extend([
                Mountain(i, mountain_h)
            ]);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        mountains.sort_by(|a, b| b.1.cmp(&a.1));
        eprintln!("{:?}", &mountains);
        println!("{}", mountains.first().unwrap().0); // The index of the mountain to fire on.
    }
}
