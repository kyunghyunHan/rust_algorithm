fn main() {
    let a = 1;
    let b = 8;
    let c = 8;

    println!("{}", maximum_score(a, b, c,));
}

pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let mut nums = vec![a, b, c];
    nums.sort();

    // let total_sum = a + b + c;
    // let max_value = nums[2];
    // let remaining_sum = nums[0] + nums[1];

    // if max_value > remaining_sum {
    //     return remaining_sum + (max_value - remaining_sum);
    // } else {
    //     return total_sum / 2;
    // }
    ((a + b + c) / 2).min(a + b + c - nums.iter().max().unwrap())
}
