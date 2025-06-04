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

fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    advanced_bubble_sort(arr, (len as i32) - 1);
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
fn advanced_bubble_sort(arr: &mut [i32], n: i32) {
    for i in 0..n - 1 {
        let mut sorted = true; //이미 정렬된 상태라 가정
        for j in 0..(n - 1) - i {
            //왼쪽애서 오른쪾으로 진행하는 경우
            if arr[j as usize] > arr[j as usize + 1] {
                arr.swap(j as usize, j as usize + 1);
                sorted = false; //자리바꿈 발생
            }
        }
        // ✅ 내부 루프가 끝난 후에 체크해야 함
        if sorted == true {
            //이미 졍렬된 상태 이므로  종료
            break;
        }
    }
}
pub fn example() {
    // 예시 사용법
    let mut numbers = vec![50, 40, 30, 20, 10];
    bubble_sort(&mut numbers);
    println!("정렬된 배열: {:?}", numbers);
    let len = numbers.len();
    advanced_bubble_sort(&mut numbers, len as i32);
    println!("정렬된 배열: {:?}", numbers);
}
