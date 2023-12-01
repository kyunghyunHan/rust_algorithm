/*힙 
힙은 이진트리로 구성 가능
min heap = 부모가 자식보다 값이 작아야함
max heap = 부모가 자식보다 값이 커야 함


*/

const MAX_SIZE: usize = 100000;


struct Heap {
    data: Vec<i32>,
    heap_size: usize,
}
impl Heap{
    fn new()->Self{
        Heap { data:vec![0;MAX_SIZE], heap_size: 0 }
    }
    //힙은 완전 이진트리의 형태로 데이터를 삽입하기에 새로운 데이터는 가장 마지막 인덱스에 저장
    fn add(&mut self ,data:i32){
        //맨 마지막에 데이터를 추가
        self.heap_size+=1;
        self.data[self.heap_size]=data;

        let mut me = self.heap_size;
        let mut parent= me/2;

    }
}
pub fn main(){

}