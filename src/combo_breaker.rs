const BIG_PRIME: u64 = 20201227;

fn find_loop_size(public_key: u64, subject: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        value *= subject;
        value %= BIG_PRIME;
        if value == public_key {
            return loop_size;
        }
    }
}

fn compute_key(loop_size: u64, subject: u64) -> u64 {
    let mut v = 1;
    for _ in 0..loop_size {
        v *= subject;
        v %= BIG_PRIME;
    }
    v
}

fn main() {
    let subject = 7;
    let door_public_key = 19241437;
    let card_public_key = 17346587;

    let door_loop_size = find_loop_size(door_public_key, subject);
    let card_loop_size = find_loop_size(card_public_key, subject);
    println!("Door loop size is: {}", door_loop_size);
    println!("Card loop size is: {}", card_loop_size);

    println!("Part 1. Encryption key handshake is {}", compute_key(door_loop_size, card_public_key));
    println!("Part 1. Encryption key handshake is {}", compute_key(card_loop_size, door_public_key));
}