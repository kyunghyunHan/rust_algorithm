fn merge(arr: &mut Vec<i32>, p: i32, q: i32, r: i32) {
    let nl = q - p + 1; //A[p:q]의 크기
    let nr = r - q; //A[q +1:r]의 크기
                    //배열 [0:nl -1] 과 R[0:nr] 를 생성
    let mut l_arr: Vec<i32> = vec![0; (nl) as usize];
    let mut r_arr: Vec<i32> = vec![0; (nr) as usize];
    for i in 0..nl {
        //A[p:q]를 L[0:nl-1]로복사
        l_arr[i as usize] = arr[(p + i) as usize]
    }
    for j in 0..nr {
        //[q+1:r]를 R[0:nr-1]로복사
        r_arr[j as usize] = arr[(q + j + 1) as usize]
    }
    let mut i = 0;//i는 L에 남아있는 가장 작은 원소를 인덱싱
    let mut j = 0;//j는 R에 남아있는 가장 작은 원소를 인덱싱
    let mut k = p;//k는 채울 A의 위치를 인덱싱
    //각 배열 L과 R에 병합되지 않은 원소가 포함되어 있으면 병합되지 않은 원소중 가장 작은 원소를 A[p:r]로 다시복사
    while i < nl && j < nr {
        if l_arr[i as usize] <= r_arr[j as usize] {
            arr[k as usize] = l_arr[i as usize];
            i += 1;
        } else {
            arr[k as usize] = r_arr[j as usize];
            j += 1;
        }
        k += 1;
    }
    //L과 R중 하나를 완전히 살펴본 후 나머지 하나를 A[p:r]끝에 복사
    while i < nl {
        arr[k as usize] = l_arr[i as usize];
        i += 1;
        k += 1;
    }

    while j < nr {
        arr[k as usize] = r_arr[j as usize];
        j += 1;
        k += 1;
    }
}
fn merge_sort(arr: &mut Vec<i32>, p: i32, r: i32) {
    if p >= r {
        return;
    }

    let q = (p + r) / 2;
    merge_sort(arr, p, q);
    merge_sort(arr, q + 1, r);
    merge(arr, p, q, r);
}
pub fn example() {
    let mut arr = vec![3, 41, 52, 26, 38, 57, 9, 49];
    let n = &arr.len();
    merge_sort(&mut arr, 0, (n - 1) as i32);
    println!("{:?}", arr);
}
