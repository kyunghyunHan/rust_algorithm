fn quick_sort(a: &mut [usize], p: &usize, r: &usize) {
    let q = partition(a, p, r);
    quick_sort(a, p, &(q - 1));
    quick_sort(a, &(q + 1), r);
}
fn partition(a: &mut [usize], p: &usize, r: &usize) -> usize {
    let x = a[*r];
    let mut i = p - 1;
    for j in *p..r - 1 {
        if a[j] <= x {
            i += 1;
            a.swap(i, j);
        }
    }
    a.swap(i + 1, *r);
    i + 1
}
pub fn example() {
    let mut a: [usize; 10] = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    let n =a.len()-1;
    quick_sort(&mut a, &0, &n);
    println!("{:?}", a);
}
