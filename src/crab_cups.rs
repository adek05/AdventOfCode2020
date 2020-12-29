use std::collections::HashMap;

fn into_linked_list(mut cups: Vec<u64>) -> HashMap<u64, u64> {
    let mut cups2 = cups.clone();
    cups2.push(cups[0]);

    cups.insert(0, *cups.last().unwrap());
    cups.into_iter().zip(cups2).collect()
}

fn find_insert_index2(mut current_cup: u64, modulo: u64, removed_range: &[u64]) -> u64 {
    loop {
        current_cup -= 1;
        if current_cup == 0 {
            current_cup = modulo;
        }
        if removed_range
            .iter()
            .find(|value| *value == &current_cup)
            .is_none()
        {
            return current_cup;
        }
    }
    panic!("Unreachable");
}

fn crab_move2(current_cup: &u64, cups: &mut HashMap<u64, u64>, modulo: u64) -> u64 {
    let mut range = vec![];
    range.push(*cups.get(current_cup).unwrap());
    range.push(*cups.get(range.last().unwrap()).unwrap());
    range.push(*cups.get(range.last().unwrap()).unwrap());

    let insert_idx = find_insert_index2(*current_cup, modulo, &range.as_slice());
    // current -> after range
    cups.insert(*current_cup, *cups.get(range.last().unwrap()).unwrap());

    let tmp = *cups.get(&insert_idx).unwrap();
    cups.insert(insert_idx, *range.first().unwrap());
    cups.insert(*range.last().unwrap(), tmp);

    *cups.get(current_cup).unwrap()
}

fn main() {
    {
        let input: Vec<u64> = vec![3, 6, 2, 9, 8, 1, 7, 5, 4];
        let input_len = input.len();

        let mut linked_list = into_linked_list(input);
        let mut current_cup = 3;

        for _ in 0..100 {
            current_cup = crab_move2(&current_cup, &mut linked_list, input_len as u64);
        }

        let mut cur = 1;
        print!("Part 1. ");
        for _ in 1..9 {
            let next = linked_list.get(&cur).unwrap();
            print!("{}", next);
            cur = *next;
        }
        println!();
    }

    {
        let mut input: Vec<u64> = vec![3, 6, 2, 9, 8, 1, 7, 5, 4];
        input.extend(10..1_000_001);

        let input_len = input.len();

        let mut linked_list = into_linked_list(input);
        let mut current_cup = 3;

        for _ in 0..10_000_000 {
            current_cup = crab_move2(&current_cup, &mut linked_list, input_len as u64);
        }

        let first = linked_list.get(&1).unwrap();
        let second = linked_list.get(first).unwrap();
        println!(
            "First: {} Second: {}. Product: {}",
            first,
            second,
            first * second
        );
    }
}
