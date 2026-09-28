use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX: usize = 102;
static mut N: usize = 0;
static mut ARR: [usize; MAX] = [0; MAX];

fn dfs(k: usize) {
    unsafe {
        print!("{} ", k);
        if (ARR[k] == 0) {
            return;
        }
        dfs(ARR[k]);
    }
}

pub fn example() {
    unsafe {
        let file = File::open("data.txt").unwrap();
        let reader = BufReader::new(file);

        let mut lines = reader.lines();

        // 첫 번째 줄
        N = lines.next().unwrap().unwrap().trim().parse().unwrap();

        loop {
            // 7 1 같은 줄 받기
            let line = lines.next().unwrap().unwrap();

            if line.trim() == "-1" {
                break;
            }

            let nums: Vec<usize> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let parent = nums[0];
            let count = nums[1];


            // 다음 줄 하나 읽기
            let child_line = lines.next().unwrap().unwrap();

            let children: Vec<usize> = child_line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for i in 0..children.len() {
                ARR[children[i]] = nums[0];
            }
        }
        dfs(N);
    }
}
