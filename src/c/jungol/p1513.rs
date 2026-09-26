/*
순열정복1
스택
*/

use std::io::{BufRead, BufReader};
static mut n: usize = 0;

static mut ARR: [char; 7] = ['0'; 7];
static mut USED: [usize; 7] = [0; 7];

fn dfs(top: usize, win: usize, lose: usize) {
    unsafe {
        if win == n {
            for i in 0..top {
                print!("{}", ARR[i]);
            }
            println!();
            return;
        }

        if lose == n {
            return;
        }

        ARR[top] = 'o';
        dfs(top + 1, win + 1, lose);

        ARR[top] = 'x';
        dfs(top + 1, win, lose + 1);
    }
}
pub fn example() {
    unsafe {
        let file = std::fs::File::open("data.txt").unwrap();
        let mut reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            n = line.next().unwrap().parse::<usize>().unwrap();
        }
        dfs(0, 0, 0);
    }
}
