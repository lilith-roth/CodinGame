use crate::structs;

fn calculate_pythagorean_theorem(a: i32, b: i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}

pub(crate) fn calculate_distance(a: structs::Position, b: structs::Position) -> i32 {
    ((((a.0 - b.0).pow(2)) + ((a.1 - b.1).pow(2))) as f32).sqrt() as i32
}

pub(crate) fn is_in_distance(a: structs::Position, b: structs::Position, distance: i32) -> bool {
    distance >= calculate_distance(a, b)
}
