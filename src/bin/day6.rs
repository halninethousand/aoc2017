use std::collections::{HashSet, HashMap};

fn main() {
    let mut input: Vec<u16> = include_str!("../../data/day6.txt").split_whitespace().map(|num| num.parse().unwrap()).collect();
    println!("length {}, {:?}", input.len(), input);

    let mut tracker: HashSet<Vec<u16>> = HashSet::new();
    let mut cycler: HashMap<Vec<u16>, u16> = HashMap::new();
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
            let insert_cycle = cycler.get(&input.clone()).unwrap();
            println!("seen before {:?}, cycle: {}, cycles since: {}", input, cycle, cycle - insert_cycle);
            break;
        }

        tracker.insert(input.clone());
        cycler.entry(input.clone()).or_insert(cycle);
    }
}
