use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
pub fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut input = String::new();
    let mut writet= BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<u64>().unwrap());
    let n =nums.next().unwrap();
    let k= nums.next().unwrap();
    let mut d: Vec<Vec<u64>> = vec![vec![0; n as usize + 1]; n as usize + 1];
    

    for i in 0..=n{
        d[i as usize][1]= i;
        d[i as usize][0]= 1;
        d[i as usize][i as usize]= 1;
    }

    for  i in 2..=n{
        for j in 1..i{
            d[i as usize][j as usize]= d[i as usize-1][j as usize]+d[i as usize-1][j as usize-1]  ; 
            d[ i as usize][j as usize]= d[i as usize][j as usize] % 10007
        }
    }
    writeln!(writet,"{}",d[n as usize][k as usize]).unwrap();
   writet.flush().unwrap();
}