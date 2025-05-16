fn solution(mut input: Vec<i32>) -> Vec<i32> {
    for i in 1..input.len() {
        if input[i - 1] > input[i] {
            input.swap(i - 1, i);
        }
    }

    input
}

pub fn example() {
    let input = vec![3, 2, 1];
    println!("{:?}", solution(input));
}
