/*
토마토 초



*/
// use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
const MAX: usize = 102;
#[derive(Copy, Clone)]
struct Point {
    z: usize,
    y: usize,
    x: usize,
}
static mut M: i32 = 0;
static mut N: i32 = 0;
static mut H: i32 = 0;
static mut DAY: i32 = 0;

const dy: [i32; 6] = [1, -1, 0, 0, 0, 0];
const dx: [i32; 6] = [0, 0, -1, 1, 0, 0];
const dz: [i32; 6] = [0, 0, 0, 0, 1, -1];
static mut USED: [[[usize; MAX]; MAX]; MAX] = [[[0; MAX]; MAX]; MAX];
static mut ARR: [[[i32; MAX]; MAX]; MAX] = [[[0; MAX]; MAX]; MAX];
static mut REAR: usize = 0;
static mut FRONT: usize = 0;
static mut QUEUE: [Point; MAX * MAX * MAX] = [Point { z: 0, y: 0, x: 0 }; MAX * MAX * MAX];

pub fn example() {
    unsafe {
        // let file = File::open("data.txt").unwrap();
        // let reader = BufReader::new(file);
        let reader = BufReader::new(stdin().lock());
        let mut lines = reader.lines();
        let first_line = lines.next().unwrap().unwrap();
        let mut iter = first_line.split_whitespace();
        (M, N, H) = (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        );

        for z in 1..=H as usize {
            for y in 1..=N as usize {
                let line = lines.next().unwrap().unwrap();

                for (x, value) in line.split_whitespace().enumerate() {
                    ARR[z][y][x + 1] = value.parse::<i32>().unwrap();
                }
            }
        }
        let mut cnt = 0;
        //1은 익은 토마노 정수 0은 익지않은 토마토 정수-1은 토마토가 들어있지않음
        for i in 1..=H {
            for j in 1..=N {
                for k in 1..=M {
                    if ARR[i as usize][j as usize][k as usize] == 1 {
                        QUEUE[REAR] = Point {
                            z: i as usize,
                            y: j as usize,
                            x: k as usize,
                        };
                        REAR += 1;
                    } else if ARR[i as usize][j as usize][k as usize] == 0 {
                        cnt += 1;
                    }
                }
            }
        }

        if cnt == 0 {
            println!("{}", 0);
            return;
        }
        bsf();
        for i in 1..=H {
            for j in 1..=N {
                for k in 1..=M {
                    if ARR[i as usize][j as usize][k as usize] == 0 {
                        println!("{}", -1);
                        return;
                    }
                }
            }
        }
        let day = DAY;
        println!("{}", day - 1);
    }
}

fn bsf() {
    unsafe {
        while FRONT < REAR {
            let mut current = QUEUE[FRONT];
            FRONT += 1;

            for i in 0..6 {
                let mut next_y = current.y as i32 + dy[i as usize];
                let mut next_z = current.z as i32 + dz[i as usize];
                let mut next_x = current.x as i32 + dx[i as usize];

                if next_x < 1 || next_x > M || next_y < 1 || next_y > N || next_z < 1 || next_z > H
                {
                    continue;
                }
                if USED[next_z as usize][next_y as usize][next_x as usize] == 1 {
                    continue;
                }
                if ARR[next_z as usize][next_y as usize][next_x as usize] == -1 {
                    continue;
                }
                if ARR[next_z as usize][next_y as usize][next_x as usize] != 0 {
                    continue;
                }
                QUEUE[REAR] = Point {
                    z: next_z as usize,
                    y: next_y as usize,
                    x: next_x as usize,
                };
                REAR += 1;
                USED[next_z as usize][next_y as usize][next_x as usize] = 1;
                ARR[next_z as usize][next_y as usize][next_x as usize] =
                    ARR[current.z][current.y][current.x] + 1;
                DAY = ARR[next_z as usize][next_y as usize][next_x as usize];
            }
        }
    }
}
