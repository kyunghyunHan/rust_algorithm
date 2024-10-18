fn merge(arr: &mut Vec<i32>, q: i32, p: i32, r: i32) {
    let nl = q - p + 1;
    let nr = r - q;
    let mut l_arr: Vec<i32> = vec![0; (nl - 1) as usize];
    let mut r_arr: Vec<i32> = vec![0; (nr - 1) as usize];
    // for i in 0..nl - 1 {
    //     l_arr[i as usize] = arr[(p + i) as usize]
    // }
    // for j in 0..nr - 1 {
    //     r_arr[j as usize] = arr[(p + j +1) as usize]
    // }
    for i in 0..nl {
        l_arr[i as usize] = arr[(p + i) as usize];
    }
    // 오른쪽 배열에 복사
    for j in 0..nr {
        r_arr[j as usize] = arr[(q + 1 + j) as usize];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < nl && j < nr {
        if l_arr[i as usize] <= r_arr[j as usize] {
            arr[k as usize] = l_arr[i as usize];
            i += 1;
        } else if arr[k as usize] == r_arr[j as usize] {
            j += 1;
        }
        k += 1;
    }

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

    let q = ((p + r) / 2);
    merge_sort(arr, p, q);
    merge_sort(arr, q + 1, r);
    merge(arr,p, q, r);
}
pub fn example() {
    let mut arr = vec![3, 41, 52, 26, 38, 57, 9, 49];
    let n = &arr.len();
    merge_sort(&mut arr, 0, (n - 1) as i32);
    println!("{:?}", arr);

}
