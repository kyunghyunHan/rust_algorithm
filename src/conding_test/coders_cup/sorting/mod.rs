fn solution(mut a: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    for i in 1..n {
        let mut index = i;
        while index > 0 && a[index] < a[index - 1] {
            a.swap(index, index - 1);
            index -= 1;
        }
    }
    a
}

fn example() {
    let a = vec![9, 2, 3, 1];
    println!("{:?}", solution(a));
}
