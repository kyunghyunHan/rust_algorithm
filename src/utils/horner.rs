/*호너의 법칙 */

fn horner(arr:&mut [usize],n:usize,x:usize)->usize{
    let mut p = 0;
    for i in (0..n).rev(){
        p = arr[i]+x*p;
    }
    p
   
}

pub fn example() {
    let mut arr = vec![31, 41, 59, 26, 41, 58];
    let n = arr.len();
    let p = horner(&mut arr, n,2);
    println!("{:?}", p);
}