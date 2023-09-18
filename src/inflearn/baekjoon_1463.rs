use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};
pub fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input= String::new();
    reader.read_line(&mut input).unwrap();
     let n :i32= input.trim().parse().unwrap();
     let mut d= [0;1000001];//d[i] =d[i-1]+1
     d[1]= 0;
     for i in 2..n+1{
        d[i as usize]= d[i as usize-1]+1;
        if i%2==0{
            d[i as usize]= std::cmp::min(d[i as usize],d[i as usize/2]+1)

        }
        if i%3==0{
            d[i as usize]= std::cmp::min(d[i as usize],d[i as usize/3]+1)
        }
     }
     writeln!(writer,"{}",d[n as usize]).unwrap();
    writer.flush().unwrap();
}
