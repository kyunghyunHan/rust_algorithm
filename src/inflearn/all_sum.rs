use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
pub fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writer= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let  nums:Vec<usize> = input.trim().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let n = nums[0];
    let m  = nums[1];
    let mut a_arr :Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    let mut arr = vec![vec![0; n+1]; n+1];
  
    for i in 0..nums[0]{
      input.clear();
      reader.read_line(&mut input).unwrap();

      let  a_row:Vec<usize> = input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();

      for j in 0..a_row.len(){
        a_arr[i][j] = a_row[j]

      }

    }
    for i in 1..n+1 {
        for j in 1..n+1 {
            arr[i][j] = arr[i][j-1] + arr[i-1][j] - arr[i-1][j-1] + a_arr[i-1][j-1];
        }
    }
    for _ in 0..m{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let nums:Vec<usize>= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect();
        let (x1,y1,x2,y2) = (nums[0],nums[1],nums[2],nums[3]);
        let result =  arr[x2][y2] - arr[x1-1][y2] - arr[x2][y1-1] + arr[x1-1][y1-1];

        writeln!(writer,"{}",result).unwrap();    
    }

}