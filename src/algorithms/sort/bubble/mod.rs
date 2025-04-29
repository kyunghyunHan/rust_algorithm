// fn bubble_sort(arr: &mut Vec<usize>, n: usize) {
//     for i in 1..n {
//         for j in (i..n).rev() {
//             if arr[j] < arr[j - 1] {
//                 arr.swap(j, j - 1);
//             }
//         }
//     }
// }

// pub fn example() {
//     let mut arr = vec![3, 5, 4, 2, 1];
//     let n = arr.len();
//     bubble_sort(&mut arr, n);
//     println!("{:?}", arr);
// }

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    bubble_sort_recursive(arr, (len as i32) - 1);
}

fn bubble_sort_recursive(arr: &mut [i32], last: i32) {
    if last > 0 {
        for i in 1..=last {
            if arr[i as usize - 1] > arr[i as usize] {
                arr.swap(i as usize - 1, i as usize);
            }
        }
        bubble_sort_recursive(arr, last - 1);
    }
}

pub fn example() {
    // 예시 사용법
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut numbers);
    println!("정렬된 배열: {:?}", numbers);
}
