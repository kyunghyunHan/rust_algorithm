mod bead_sort;


fn main() {
    let mut test:[usize;5]= [1,5,5,6,1];
  let tt=   bead_sort::bead_sort(&mut test);
    println!("{:?}",tt);
}
