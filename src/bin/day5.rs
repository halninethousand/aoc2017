fn main() {
    let mut input: Vec<i32> = include_str!("../../data/day5.txt").lines().map(|n| n.parse().unwrap()).collect();
    let mut input_2 = input.clone(); 

    let mut pointer: i32 = 0;
    let length = input.len() as i32;
    let mut steps = 1;

    'main: loop {
        let value = input[pointer as usize]; 
        if pointer + value >= length {
            println!("Part1: we got out in {} steps", steps);
            break 'main;
        } else {
            input[pointer as usize] += 1;
            pointer += value;
            steps += 1;
        }
    }

    steps = 1;
    pointer = 0;

    'main: loop {
        let value = input_2[pointer as usize]; 
        if pointer + value >= length {
            println!("Part2: we got out in {} steps", steps);
            break 'main;
        } else {
            if input_2[pointer as usize] >= 3 {
                input_2[pointer as usize] -= 1;
            } else {
                input_2[pointer as usize] += 1;
            }
            pointer += value;
            steps += 1;
        }
    }
}
