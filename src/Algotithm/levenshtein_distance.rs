/*스트링 편집거리 알고리즘 */

pub fn main(){
    let a = "ca".trim();
    let b = "abc".trim();
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];



    for i in 0..=a.len() {
        dp[i][0] = i;
    }
    for i in 0..=b.len() {
        dp[0][i] = i;
    }

    for from in 1..=a.len() {
        for to in 1..=b.len() {
            if a.chars().nth(from - 1) == b.chars().nth(to - 1) {
                dp[from][to] = dp[from - 1][to - 1];
            } else {
                dp[from][to] = dp[from - 1][to - 1]
                    .min(dp[from - 1][to])
                    .min(dp[from][to - 1])
                    + 1;
            }
        }
    }

    println!("{}", dp[a.len()][b.len()]);
}

