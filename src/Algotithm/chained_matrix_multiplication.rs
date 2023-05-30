/*연쇄행렬곱셈 */

fn solution(matrix_sizes:Vec<Vec<i32>>)->i32{
    let size= matrix_sizes.len();
    let mut db= vec![vec![999999999;size];size];

    for i in 0..size{
        dp[i][i]= 0;
    }

    for i in 0..size{
        for j in 0..(size-i){
            if j==j+i{
                continue;
            }

            let a= j;
            let b= j+i;
            for k in a..b{
                dp[a][b]= dp[a][b].min()
            }
        }
    }



}