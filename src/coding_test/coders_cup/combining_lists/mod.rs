pub fn solution(input_list_one: Vec<i32>, input_list_two: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..input_list_one.len() {
        v.push(input_list_one[i]);
        v.push(input_list_two[i]);
    }
    v
}

pub fn example() {
    let input_list_one = vec![1, 1, 1, 1];
    let input_list_two = vec![2, 2, 2, 2];

    println!("{:?}", solution(input_list_one, input_list_two));
}
