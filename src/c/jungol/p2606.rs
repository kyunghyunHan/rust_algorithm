/*
토마토 초



*/
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
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
        let file = File::open("data.txt").unwrap();
        let reader = BufReader::new(file);
        let mut line = reader.lines();
        let mut iter = line.next().unwrap().unwrap();
        let mut iter = iter.split_whitespace();
        (M, N, H) = (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        );
        for i in 1..=H {
            let line = iter.next().unwrap();
            // let mut iter = line.split_whitespace();
            let mut nums = [0i32; MAX];

            for (i, x) in line.split_whitespace().enumerate() {
                nums[i] = x.parse::<i32>().unwrap();
            }
            for i in 0..N {
                ARR[H as usize][M as usize] = nums;
            }
        }
        //1은 익은 토마노 정수 0은 익지않은 토마토 정수-1은 토마토가 들어있지않음
        for i in 1..=H {
            for j in 1..=M {
                for k in 1..=N {
                    if ARR[i as usize][j as usize][k as usize] == 1 {
                        QUEUE[REAR] = Point {
                            z: i as usize,
                            y: j as usize,
                            x: k as usize,
                        };
                        REAR += 1;
                    }
                }
            }
        }

        bsf();
    }
}

fn bsf() {
    let mut day = 0;
    unsafe {
        while FRONT < REAR {
            let mut current = QUEUE[FRONT];
            FRONT += 1;

            for i in 0..6 {
                for j in 0..6 {
                    for k in 0..6 {
                        // let next_h =

                        let mut next_y = current.y + dy[i as usize] as usize;
                        let mut next_z = current.z + dz[i as usize] as usize;
                        let mut next_x = current.x + dx[i as usize] as usize;

                        if next_x < 1
                            || next_x > M as usize
                            || next_y < 1
                            || next_y > N as usize
                            || next_z < 1
                            || next_z > H as usize
                        {
                            continue;
                        }
                        if USED[next_z][next_y][next_x] == 1 {
                            continue;
                        }
                        if ARR[next_z][next_y][next_x] == -1 {
                            continue;
                        }

                        QUEUE[REAR] = Point {
                            z: next_z,
                            y: next_y,
                            x: next_x,
                        };
                        REAR += 1;
                        USED[next_z][next_y][next_x] = 1;
                        ARR[next_z][next_y][next_x] = ARR[current.z][current.y][current.x] + 1;
                        day = ARR[next_z][next_y][next_x];
                    }
                }
            }
        }
    }
}
