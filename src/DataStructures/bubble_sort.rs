/*버블정렬 */
fn bubble_sort(a: &mut [i32]) {
    let n = a.len() as i32;
    for i in 0..n-1 {
        for j in 0..n-1-i {
            if a[j] > a[j+1] {
                a.swap(j, j+1);
            }
        }
    }
}



pub fn main(){}