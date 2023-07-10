use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let opponent_count = parse_input!(input_line, i32); // Opponent count

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let game_round = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32); // Your x position
        let y = parse_input!(inputs[1], i32); // Your y position
        let back_in_time_left = parse_input!(inputs[2], i32); // Remaining back in time
        for i in 0..opponent_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let opponent_x = parse_input!(inputs[0], i32); // X position of the opponent
            let opponent_y = parse_input!(inputs[1], i32); // Y position of the opponent
            let opponent_back_in_time_left = parse_input!(inputs[2], i32); // Remaining back in time of the opponent
        }
        for i in 0..20 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.trim().to_string(); // One line of the map ('.' = free, '0' = you, otherwise the id of the opponent)
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // action: "x y" to move or "BACK rounds" to go back in time
        println!("17 10");
    }
}

