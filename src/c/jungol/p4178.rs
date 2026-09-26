/*
순열 정복2


*/
use std::fs::File;
use std::io::{BufRead, BufReader};
static mut ARR: [i32; 8] = [0; 8];
static mut USED: [i32; 8] = [0; 8];
static mut target: i32 = 0;
static mut n: i32 = 0;
unsafe fn dfs(top: i32) {
    if top == target {
        for i in 0..target {
            print!("{} ", ARR[i as usize]);
        }
        println!();
        return;
    }

    for i in 1..=n {
        if USED[i as usize] == 1 {
            continue;
        }
        USED[i as usize] = 1;
        ARR[top as usize] = i;
        dfs(top + 1);
        USED[i as usize] = 0;
    }
}
pub fn example() {
    unsafe {
        let file = File::open("data.txt").unwrap();
        let reader = BufReader::new(file);

        // let mut input = String::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            n = line.next().unwrap().parse::<i32>().unwrap();
            target = line.next().unwrap().parse::<i32>().unwrap();

            dfs(0);
        }
    }
}
