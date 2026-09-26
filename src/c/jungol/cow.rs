use std::fs::File;
use std::io::{BufRead, BufReader};

fn binary_search(arr: Vec<i32>, mut left: i32, mut right: i32, target: i32, upper: i32) -> i32 {
    // 탐색 범위는 [left, right)이다.
    // 즉, left는 포함하지만 right는 포함하지 않는다.
    while (left < right) {
        // 현재 탐색 범위의 가운데 인덱스
        let mid = left + (right - left) / 2;

        // upper == 0인 경우:
        //   target 이상인 값이 처음 나타나는 위치(lower bound)를 찾는다.
        //   arr[mid] == target이어도 정답이 더 왼쪽에 있을 수 있으므로 right를 줄인다.
        //
        // upper == 1인 경우:
        //   target보다 큰 값이 처음 나타나는 위치(upper bound)를 찾는다.
        //   arr[mid] == target이면 해당 값까지 포함해야 하므로 left를 오른쪽으로 옮긴다.
        if arr[mid as usize] > target || (upper == 0 && arr[mid as usize] == target) {
            // mid가 정답일 수도 있으므로 탐색 범위에 mid를 남겨 둔다.
            right = mid;
        } else {
            // mid는 정답이 될 수 없으므로 mid 다음 위치부터 탐색한다.
            left = mid + 1;
        }
    }

    // left == right가 되면 조건을 만족하는 첫 번째 위치가 된다.
    // 조건을 만족하는 값이 없다면 배열의 끝 인덱스(right의 초기값)를 반환한다.
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
