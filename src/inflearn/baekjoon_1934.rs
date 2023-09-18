use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
pub fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
     let n :i32= input.trim().parse().unwrap();
     for i in 0..n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse::<u64>().unwrap());
        let a= nums.next().unwrap();
        let b= nums.next().unwrap();
        //a*b/최대공약수 
        let mut result =a*b/gcd(a, b);

        writeln!(writer,"{}",result).unwrap();
      
     }
}
fn gcd(a:u64,b:u64)->u64{
    if b==0{
        return a;
    }else{
        return gcd(b,a%b)
    }

}