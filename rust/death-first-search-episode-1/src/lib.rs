use std::collections::BTreeMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, Clone, Copy)]
struct Gateway(i32, i32);

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], i32); // the total number of nodes in the level, including the gateways
    let l = parse_input!(inputs[1], i32); // the number of links
    let e = parse_input!(inputs[2], i32); // the number of exit gateways
    let mut b_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for i in 0..l as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n1 = parse_input!(inputs[0], i32); // N1 and N2 defines a link between these nodes
        let n2 = parse_input!(inputs[1], i32);
        eprintln!("i {}", i);
        eprintln!("n1 {}", n1);
        eprintln!("n2 {}", n2);


        let mut old_entry: Vec<i32> = Clone::clone(&b_map).get_mut(&n1)
            .unwrap_or(&mut vec![]).to_owned();
        old_entry.extend([n2]);
        b_map.insert(n1, old_entry);
    }
    let mut gateways: Vec<Gateway> = vec![];
    for i in 0..e as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let ei = parse_input!(input_line, i32); // the index of a gateway node
        eprintln!("i2 {}", i);
        eprintln!("ei {} {}", i, ei);
        gateways.extend([Gateway(i as i32, ei)]);
    }
    eprintln!("gateways {:?}", gateways);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let si = parse_input!(input_line, i32); // The index of the node on which the Bobnet agent is positioned this turn
        eprintln!("si {}", si);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // Example: 0 1 are the indices of the nodes you wish to sever the link between
        let gateway_item = gateways.first().unwrap();
        eprintln!("gi {:?}", gateway_item);
        eprintln!("map {:?}", b_map);
        let map_item = b_map.get_mut(&gateway_item.0).unwrap().pop();
        // eprintln!("mi {}", map_item);
        println!("{:?} {:?}",
                //  gateway_item.1,
                 map_item.unwrap(),
                  gateway_item.1,
        );
    }
}
