mod DataStructures;


fn main() {
    let mut test:[usize;5]= [1,5,5,6,1];
  let tt=   DataStructures::bead_sort::bead_sort(&mut test);
    println!("{:?}",tt);
}
