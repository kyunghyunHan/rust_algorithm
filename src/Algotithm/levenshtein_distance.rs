/*스트링 편집거리 알고리즘 */





fn ed(n:i32,x:&[i32],m:i32,y:&[i32],ins:i32,del:i32,chg:i32){
    let mut e= vec![vec![0; x.len() + 1]; y.len() + 1];
    e[0][0]= 0;

    
    //삽입비용:ins
    //삭제비용:del,
    //변경비용:chg
    for i in 1..=n{

        e[i as usize][0]= e[i as usize-1][0]+del;//첫열의 초기화
    }
    for j in 1..=m{

        e[0][j as usize]= e[0][j as usize-1]+ins;//첫행의 초기화
    }
    for i in 1..=n{
        for j in 1..=m{
            /*
            c= x[i]==y[j]?0:chg;
            e[i][j]= min(e[i-1][j]+del,e[i][j-1]+ins,e[i-1][j-1]+c);
            return e[n][m]
             */
        }
    }

}
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

