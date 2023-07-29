pub(crate) fn are_all_remaining_zombies_grouped(zombies: &[structs::Zombie]) -> bool {
    let mut grouped = true;

    'outer: for (i, zombie) in zombies.iter().enumerate() {
        for other_zombie in zombies.iter().skip(i + 1) {
            eprintln!("comp: {:?} {:?} {}",
                      zombie.entity.position,
                      other_zombie.entity.position,
                      math::calculate_distance(
                          zombie.entity.position,
                          other_zombie.entity.position,
                      )
            );
            if !math::is_in_distance(
                zombie.entity.position,
                other_zombie.entity.position,
                1500,
            ) {
                grouped = false;
                break 'outer;
            }
        }
        if !grouped {
            break;
        }
    }

    grouped
}