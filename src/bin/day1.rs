fn main() {
    let input: Vec<u16> = include_str!("../../data/day1.txt").trim().chars().map(|n| n.to_string().trim().parse::<u16>().unwrap()).collect();

    println!("{input:?}");

    let mut sum = 0;

    for i in 0..input.len() {
        if i == input.len() - 1 && input[i] == input[0] {
            sum += input[i];
        } else if input[i] == input[i+1] {
            sum += input[i]; 
        }
    }

    println!("{sum:?}");

}
