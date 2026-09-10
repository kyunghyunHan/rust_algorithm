fn solution(input_list: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut lcnt = 0;
    let mut hcnt = 0;
    for i in 1..input_list.len() {
        if input_list[i] > input_list[i - 1] {
            hcnt += 1;
        } else if input_list[i] < input_list[i - 1] {
            lcnt += 1;
        }
    }
    total = lcnt * hcnt;
    total
}

pub fn example() {
    let input_list = vec![5, 3, 2, 1, 6, 8, 8, 9];

    println!("{}", solution(input_list));
}
