fn stalin_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for &x in &arr {
        if result.is_empty() || x >= *result.last().unwrap() {
            result.push(x);
        }
    }

    result
}

pub fn example() {
    let a = vec![1, 4, 2, 1, 4];

    let b = stalin_sort(a);

    println!("{:?}", b);
    println!("{:?}",b);
}
