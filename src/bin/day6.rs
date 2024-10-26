use std::collections::HashSet;

fn main() {
    let mut input: Vec<u16> = include_str!("../../data/day6.txt").split_whitespace().map(|num| num.parse().unwrap()).collect();
    println!("length {}, {:?}", input.len(), input);

    let mut tracker: HashSet<Vec<u16>> = HashSet::new();
    tracker.insert(input.clone());

    let mut cycle = 0;

    loop {
        cycle += 1;
        let max = *input.iter().max().unwrap();
        let mut index = input.iter().position(|element| element == &max).unwrap();
        input[index] = 0;

        for _ in 0..max {
            index = (index + 1) % input.len();
            input[index] += 1;
        }

        if tracker.contains(&input) {
            println!("seen before {:?}, cycle: {}", input, cycle);
            break;
        }

        tracker.insert(input.clone());

    }
}
