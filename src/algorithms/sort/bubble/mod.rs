fn bubble_sort(arr: &mut [i32], n: usize) {
    for i in 0..n - 1 {
        //(N-1)번 반복
        for j in 0..n - 1 {
            //왼쪾에서 오른쪽으로 진행하는 경우
            if arr[j] > arr[j + 1] {
                //왼쪾 데이터 >오른쪽 데이터 이면
                arr.swap(j, j + 1);
            }
        }
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
    let len = numbers.len();

    bubble_sort(&mut numbers, len);
    println!("정렬된 배열: {:?}", numbers);

    advanced_bubble_sort(&mut numbers, len as i32);
    println!("정렬된 배열: {:?}", numbers);
}
