fn solution(foods: &Vec<i32>) -> i32 {
    let n = &foods.len();

    let total = foods.iter().sum::<i32>();

    if total % 3 != 0 {
        0
    } else {
        let part_sum = total / 3;
        let two_part_sum = 2 * part_sum;

        let mut x = 0;
        let mut y = 0;
        let mut prefix_sum = 0;

        for i in 0..n - 1 {
            prefix_sum += foods[i];
            //두번쨰 경게에서 첫번쨰 경계갯구를 더하기
            if prefix_sum == two_part_sum {
                y += x;
            }
            //첫번쨰 경계에서 x의 개수를 세기
            if prefix_sum == part_sum {
                x += 1;
            }
        }
        y
    }
}
fn main() {
    let food1 = vec![1, 2, 3, 0, 3];
    println!("{}", solution(&food1));

    let food2 = vec![4, 1];
    println!("{}", solution(&food2));
}
