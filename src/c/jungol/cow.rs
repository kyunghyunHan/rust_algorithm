use std::fs::File;
use std::io::{BufRead, BufReader};

fn binary_search(arr: Vec<i32>, mut left: i32, mut right: i32, target: i32, upper: i32) -> i32 {
    while (left < right) {
        let mid = left + (right - left) / 2;

        if arr[mid as usize] > target || (upper == 0 && arr[mid as usize] == target) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return left;
}
pub fn example() {
    let mut cnt = 0;
    let file = File::open("data.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut arr = vec![0; n as usize];
    for i in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s1: i32 = input.trim().parse().unwrap();
        arr[i as usize] = s1;
    }

    arr.sort_by(|a, b| a.cmp(&b));
    //첫번쨰
    for i in 0..n - 2 {
        let mut lower = i + 2;
        let mut upper = i + 2;
        //두번쨰쨰
        for j in i + 1..n - 1 {
            //첫번쨰 공의 이동거리
            let d1 = arr[j as usize] - arr[i as usize];
            //두번쨰거리는 min 이상 max

            //arr[j] + d1 <= Z의 위치 <= arr[j] + 2*d1
            let min = arr[j as usize] + d1;
            let max = arr[j as usize] + d1 * 2;
            //세번쨰 소는 반드시 j보다 오른쪽

            let first = binary_search(arr.clone(), j + 1, n, min, 0);
            let last = binary_search(arr.clone(), j + 1, n, max, 1);

            cnt += last - first;

            // if lower < j + 1 {
            //     lower = j + 1;
            // }
            // //min이상인 값이 처음나오는 위치
            // while lower < n && arr[lower as usize] < min {
            //     lower += 1;
            // }
            // //upper은 lower보다 왼쪽에 있으면안댐
            // if upper < lower {
            //     upper = lower;
            // }
            // //max이상인 값이 처음나오는 위치
            // while upper < n && arr[upper as usize] <= max {
            //     upper += 1;
            // }
            // cnt += upper - lower;
        }
    }
    println!("{}", cnt);
}
