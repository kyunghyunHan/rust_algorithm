use std::{
    io::{stdin, BufRead, BufReader},
    thread::current,
};

const MAX: usize = 26;
#[derive(Clone, Copy)]
struct Point {
    r: i32,
    c: i32,
}

pub fn example() {
    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, -1, 1];
    let mut ans = [0; MAX * MAX];
    let mut house_count = 0;
    let mut queue: [Point; MAX * MAX] = [Point { r: 0, c: 0 }; MAX * MAX];

    let mut used = [[0; MAX]; MAX];
    let mut arr = [[0; MAX]; MAX];

    let mut rear = 0;
    let mut front = 0;

    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse::<i32>().unwrap();

    for i in 1..=n {
        input.clear();
        reader.read_line(&mut input).unwrap();

        for (j, x) in input.trim().bytes().enumerate() {
            arr[i as usize][j + 1] = (x - b'0') as i32;
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            if arr[i as usize][j as usize] == 1 && used[i as usize][j as usize] == 0 {
                front = 0;
                rear = 0;

                let mut count = 1;

                used[i as usize][j as usize] = 1;
                queue[rear].r = i as i32;
                queue[rear].c = j as i32;
                rear += 1;

                while front < rear {
                    let current = queue[front];
                    front += 1;

                    for d in 0..4 {
                        let nr = current.r + dr[d];
                        let nc = current.c + dc[d];

                        if nr < 1 || nr > n || nc < 1 || nc > n {
                            continue;
                        }

                        if arr[nr as usize][nc as usize] == 1 && used[nr as usize][nc as usize] == 0
                        {
                            used[nr as usize][nc as usize] = 1;
                            queue[rear].r = nr;
                            queue[rear].c = nc;
                            rear += 1;
                            count += 1;
                        }
                    }
                }
                ans[house_count] = count;
                house_count += 1;
            }
        }
    }
    ans[..house_count].sort_by(|a, b| a.cmp(b));

    for i in 0..house_count {
        println!("{}", ans[i]);
    }
}
