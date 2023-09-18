use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("Invalid input. Please enter a non-negative integer.");

    let mut d: Vec<u64> = vec![0; 92];
    d[0] = 0;
    d[1] = 1;

    for i in 2..=n {
        d[i as usize] = d[(i - 1) as usize] + d[(i - 2) as usize];
    }

    println!("{}", d[n as usize]);
}
