use std::{
    io::{stdin, BufRead, BufReader},
    ptr::null,
};

/*
토마토 (초)

입력파일 첫줄 정수 N M H

1은 익은 토마토
정수 0 은 익지 않은 토마토
정수 -1 은 토마토가 없음

위아래

저장될떄부터 모두  익어있으면 0
토마토가 모두 익지 못하면 -1
익을떄까지 몇 일걸리나

*/
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
const MAX: usize = 1002;
const dy: [i32; 4] = [1, -1, 0, 0];
const dx: [i32; 4] = [0, 0, 1, -1];

pub fn example() {
    let mut queue: [Point; MAX * MAX] = [Point { x: 0, y: 0 }; MAX * MAX];
    let mut front = 0;
    let mut rear = 0;
    let mut arr = [[0; MAX]; MAX];
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_line(&mut input).unwrap();
    let mut a = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = a.next().unwrap();
    let m = a.next().unwrap();

    for i in 1..=m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut a = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for j in 1..=n {
            arr[i][j] = a[j - 1];
        }
    }

    for i in 1..m {
        for j in 1..n {
            if arr[i][j] == 1 {
                queue[rear] = Point {
                    x: i as i32,
                    y: j as i32,
                };
            }
        }
    }

    while front < rear {
        let mut current = queue[front];
        front += 1;

        for i in 0..4 {
            let next_x = current.x + dx[i];
            let next_y = current.y = dy[i];
        }
    }

    // for i in 1..=m {
    //     for j in 1..=n {
    //         print!("{}", arr[i][j]);
    //     }
    //     print!("\n");
    // }
}
