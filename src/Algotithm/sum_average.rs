fn sum_average(a:&[usize],n:usize){



let mut sum = 0;

let mut i =0;
while i<n{
    sum =sum+a[i];
    i = i+1;
}
let average= sum/n;
println!("{} {}",sum,average);
}

pub fn main(){
   let test= [1,3,5,6,6];

   sum_average(&test,4);

}