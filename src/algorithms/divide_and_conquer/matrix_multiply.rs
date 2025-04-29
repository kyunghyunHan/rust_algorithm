//정사각형 행렬 곱하기
fn matrix_multiply(
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
fn partition_matrices(
    a: &Vec<Vec<usize>>,
    b: &Vec<Vec<usize>>,
    c: &Vec<Vec<usize>>,
    n: usize,
) -> (
    (
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
    ),
    (
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
    ),
    (
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
        Vec<Vec<usize>>,
    ),
) {
    let mid = n / 2;

    // A 행렬 파티셔닝
    let a11 = (0..mid)
        .map(|i| (0..mid).map(|j| a[i][j]).collect())
        .collect();
    let a12 = (0..mid)
        .map(|i| (mid..n).map(|j| a[i][j]).collect())
        .collect();
    let a21 = (mid..n)
        .map(|i| (0..mid).map(|j| a[i][j]).collect())
        .collect();
    let a22 = (mid..n)
        .map(|i| (mid..n).map(|j| a[i][j]).collect())
        .collect();

    // B 행렬 파티셔닝
    let b11 = (0..mid)
        .map(|i| (0..mid).map(|j| b[i][j]).collect())
        .collect();
    let b12 = (0..mid)
        .map(|i| (mid..n).map(|j| b[i][j]).collect())
        .collect();
    let b21 = (mid..n)
        .map(|i| (0..mid).map(|j| b[i][j]).collect())
        .collect();
    let b22 = (mid..n)
        .map(|i| (mid..n).map(|j| b[i][j]).collect())
        .collect();

    // C 행렬 파티셔닝
    let c11 = (0..mid)
        .map(|i| (0..mid).map(|j| c[i][j]).collect())
        .collect();
    let c12 = (0..mid)
        .map(|i| (mid..n).map(|j| c[i][j]).collect())
        .collect();
    let c21 = (mid..n)
        .map(|i| (0..mid).map(|j| c[i][j]).collect())
        .collect();
    let c22 = (mid..n)
        .map(|i| (mid..n).map(|j| c[i][j]).collect())
        .collect();

    (
        (a11, a12, a21, a22),
        (b11, b12, b21, b22),
        (c11, c12, c21, c22),
    )
}
fn pad_matrix(matrix: &Vec<Vec<usize>>, new_size: usize) -> Vec<Vec<usize>> {
    let old_size = matrix.len();
    let mut padded = vec![vec![0; new_size]; new_size];
    for i in 0..old_size {
        for j in 0..old_size {
            padded[i][j] = matrix[i][j];
        }
    }
    padded
}

fn unpad_matrix(matrix: Vec<Vec<usize>>, original_size: usize) -> Vec<Vec<usize>> {
    matrix.into_iter()
        .take(original_size)
        .map(|row| row.into_iter().take(original_size).collect())
        .collect()
}
fn matrix_multiply_recursive(
    a: &mut Vec<Vec<usize>>,
    b: &mut Vec<Vec<usize>>,
    c: &mut Vec<Vec<usize>>,
    n: usize,
) {
    if n == 1 {
        c[0][0] += a[0][0] * b[0][0];
        return;
    }
    let (
        (mut a11, mut a12, mut a21, mut a22),
        (mut b11, mut b12, mut b21, mut b22),
        (mut c11, mut c12, mut c21, mut c22),
    ) = partition_matrices(&a, &b, &c, n);
    matrix_multiply_recursive(&mut a11, &mut b11, &mut c11, n / 2);
    matrix_multiply_recursive(&mut a11, &mut b12, &mut c12, n / 2);
    matrix_multiply_recursive(&mut a21, &mut b11, &mut c21, n / 2);
    matrix_multiply_recursive(&mut a21, &mut b12, &mut c22, n / 2);
    matrix_multiply_recursive(&mut a12, &mut b21, &mut c11, n / 2);
    matrix_multiply_recursive(&mut a12, &mut b22, &mut c12, n / 2);
    matrix_multiply_recursive(&mut a22, &mut b21, &mut c21, n / 2);
    matrix_multiply_recursive(&mut a22, &mut b22, &mut c22, n / 2);
}
pub fn example() {
    let n = 3;
    
    let mut a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
    let mut c = vec![vec![0; n]; n];
    
    matrix_multiply_recursive(&mut a, &mut b, &mut c, n);
    // matrix_multiply(&mut a, &mut b, &mut c, n);

    println!("{:?}", c);
}
