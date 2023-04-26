use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Clone)]
struct EntryDistance {
    distFromZero: i32,
    index: i32,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let mut last_input: Option<EntryDistance> = None;
    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);
        eprintln!("{:?}", t);
        if last_input.is_none() {
            last_input = Option::from(EntryDistance{
                distFromZero: t,
                index: parse_input!(i, i32),
            });
            continue;
        }
        let mut test_distance = last_input.clone().unwrap().distFromZero;
        let mut input_distance;
        if test_distance < 0 {
            test_distance = -test_distance;
        }
        if t < 0 { input_distance = -t; }
        else { input_distance = t;}

        if input_distance < test_distance || (input_distance == test_distance && t > 0) {
            last_input = Option::from(EntryDistance{
                distFromZero: t,
                index: parse_input!(i, i32)
            });
        }
    }


    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    // let output: String;
    let output: String = match last_input {
        None => { "0".parse().unwrap() }
        Some(_) => { last_input.unwrap().index.to_string() }
    };
    println!("{}", output);
}
