use itertools::Itertools;

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../../data/day4.txt").lines().map(|s| s.split_whitespace().collect()).collect();

    println!("{input:?}");

    let mut valid: u16 = 0;

    for item in &input {
        let count = item.iter().unique().count();
        if count == item.len() {
            valid += 1;
        }
    }

    println!("Valid passphrases Part1: {}", valid);

    valid = 0;

    'line: for item in input {
        let comb: Vec<_> = item.iter().combinations(2).collect();

        for pair in comb {
            let mut first: Vec<char> = pair[0].chars().collect();
            let mut second: Vec<char> = pair[1].chars().collect();
            first.sort();
            second.sort();

            if first == second {
                continue 'line;
            }
        }

        valid += 1;
    }

    println!("Valid passphrases Part2: {}", valid);
}
