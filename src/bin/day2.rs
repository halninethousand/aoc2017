use itertools::Itertools;

fn main() {
    let input: Vec<Vec<u16>> = include_str!("../../data/day2.txt")
        .lines()
        .map(|line|
            line.split_whitespace().map(|num|
                num.parse::<u16>().unwrap()
                )
                .collect()
        )
        .collect();

    let mut checksum: u16 = 0;

    for line in &input {
        checksum += line.iter().max().unwrap() - line.iter().min().unwrap()
    } 

    println!("Part1: {checksum}");

    checksum = 0;

    for line in input {
        let comb = line.into_iter().combinations(2);
        let mut combinations: Vec<Vec<u16>> = comb.collect();

        for item in combinations.iter_mut() {
            item.sort();
            if item[1] % item[0] == 0 {
                checksum += item[1] / item[0];
            }
        }
        
    }

    println!("Part2: {checksum}");

}
