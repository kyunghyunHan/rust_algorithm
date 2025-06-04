fn shell_sort(arr: &mut [i32], n: usize) {
    // D = n/2부터 시작해서 D >= 1까지, D = D/2로 감소
    let mut d = n / 2;
    
    while d >= 1 {
        // D개의 부분배열에 대한 삽입 정렬 과정
        for i in d..n {
            let val = arr[i];  // 현재 삽입할 값
            let mut j = i;
            
            // j >= D이고 A[j-D] > val인 동안 반복
            while j >= d && arr[j - d] > val {
                arr[j] = arr[j - d];  // 값을 D만큼 뒤로 이동
                j -= d;
            }
            
            arr[j] = val;  // 적절한 위치에 val 삽입
        }
        
        // 간격을 절반으로 줄임
        d /= 2;
    }
}

pub fn example() {
    let mut arr = vec![5, 2, 9, 1, 5, 6];
    let n = arr.len();
    shell_sort(&mut arr, n);
    println!("{:?}", arr);
}
