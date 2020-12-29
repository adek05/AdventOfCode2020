fn find_insert_idx(current_cup: &u32, circle: &[u32], modulo: &u32) -> usize {
    let mut current_cup: u32 = *current_cup;
    loop {
        current_cup = (current_cup + modulo - 1) % modulo;
        if current_cup == 0 {
            current_cup = 9;
        }
        if let Some(idx) = circle.iter().position(|cup| cup == &current_cup) {
            return idx;
        }
    }
}

fn crab_move(current_cup: &u32, mut circle: Vec<u32>) -> (u32, Vec<u32>) {
    if let Some(idx) = circle.iter().position(|cup| cup == current_cup) {
        let total_cups = circle.len();
        let mut removed_cups: Vec<u32> = vec![];
        circle.extend(&circle.clone());
        for _ in idx + 1..idx + 4 {
            removed_cups.push(circle.remove(idx + 1));
        }
        circle = circle[idx..(idx + total_cups - 3)].to_vec();

        let insert_idx = find_insert_idx(current_cup, &circle, &(total_cups as u32));

        let tail = circle.split_off(insert_idx + 1);
        circle.extend(removed_cups);
        circle.extend(tail);
        return (circle[1], circle);
    }
    panic!("Unreachable")
}

fn main() {
    // let mut input: Vec<u32> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    let mut input: Vec<u32> = vec![3, 6, 2, 9, 8, 1, 7, 5, 4];
    let mut current_cup = 3;
    for _ in 0..100 {
        let res = crab_move(&current_cup, input);
        current_cup = res.0;
        input = res.1;
    }
    println!("Part 1. After 100 crab moves {:?}", &input);
}
