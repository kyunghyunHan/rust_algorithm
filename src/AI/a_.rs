use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::i32;
#[derive(Clone, Copy,PartialEq, Eq)]
struct Pos{
   y:usize,
   x:usize
}
#[derive(Copy,Clone,Eq,PartialEq)]
struct PQNode{
    f:i32,
    g:i32,
    pos:Pos
}

impl Ord for PQNode{
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for PQNode{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Board{
    size:usize,
    dest_y :usize,
    dest_x:usize,
    tile:Vec<Vec<TileType>>
}

enum  TileType{
    Empty,
    Wall,
} 
pub fn main(){


    println!("{}","hello a*  algorthm")



}