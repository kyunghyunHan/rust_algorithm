//정사각형 행렬 곱하기
fn matrix_multiplay(
    a: &mut Vec<Vec<usize>>,
    b: &mut Vec<Vec<usize>>,
    c: &mut Vec<Vec<usize>>,
    n: usize,
) {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }
}

pub fn example() {
    let n = 3;
    let mut a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
    let mut c = vec![vec![0; n]; n];

    matrix_multiplay(&mut a, &mut b, &mut c, n);

    println!("{:?}", c);
}
