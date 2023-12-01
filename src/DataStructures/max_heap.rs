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
         // 맨 마지막에 데이터를 추가
         self.heap_size += 1;
         self.data[self.heap_size] = data;
 
         let mut me = self.heap_size;
         let mut parent = me / 2;
 
         // 부모가 자식보다 값이 작은지 확인하고 위치 교환
         while parent > 0 {
             if self.data[parent] < self.data[me] {
                 self.data.swap(parent, me);
                 me = parent;
                 parent = me / 2;
             } else {
                 break;
             }
         }
    }
    //힙이 비어있는지 확인하는 함수
    fn is_empty(&self) -> bool {
        self.heap_size == 0
    }
    //삭제 함수
    fn remove(&mut self) -> Option<i32> {
            if self.is_empty() {
                return None;
            }
    
            // 가장 큰 값을 반환하고 삭제
            let res = Some(self.data[1]);
    
            self.data[1] = self.data[self.heap_size];
            self.heap_size -= 1;
    
            let mut me = 1;
            let mut child = 2;
    
            // 자식의 값과 비교하여 위치 교환
            while child <= self.heap_size {
                //오른쪽 자식의 인덱스 번호가 힙 크기 이하이면서 왼쪽 자식보다 더 큰 경우
                if child + 1 <= self.heap_size && self.data[child] < self.data[child + 1] {
                    child += 1;
                }
    
                if self.data[me] < self.data[child] {
                    self.data.swap(me, child);
                    me = child;
                    child = me * 2;
                } else {
                    break;
                }
            }
    
            res
        }


}
pub fn main(){
    let mut h = Heap::new();

    h.add(3);
    h.add(5);
    h.add(1);
    h.add(23);
    h.add(9);

    while let Some(value) = h.remove() {
        print!("{} ", value);
    }
}